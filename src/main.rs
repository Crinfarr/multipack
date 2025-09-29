mod platforms;

use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Write},
    sync::Arc,
};

use crate::platforms::mr::{
    ModrinthEnvironmentRequirement, ModrinthHashInfo, ModrinthJarMeta, ModrinthMeta,
};
use crate::platforms::{
    curse::{CurseGetModFileResponse, CurseHashAlgo, CurseforgeMeta},
    mr::ModrinthGetVersionFromHashResponse,
};
use reqwest::header::{HeaderMap, HeaderValue};
use tokio::{
    sync::{Mutex, Semaphore},
    task::JoinSet,
};
use tracing::{Instrument, Level, event, span};
use zip::{
    read::ZipFile,
    write::SimpleFileOptions,
};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    let mut args = std::env::args();
    args.next(); //HACK skip the exe you dumb motherfucker i hate you (me)
    let pack_filename = args.next().ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "No file specified",
    ))?;
    let mut pack_split: Vec<&str> = pack_filename.split(".").collect();
    let pack_ext = pack_split.pop().unwrap();
    let pack_name = &pack_split.join(".");
    let i_file = File::open(&pack_filename)?;
    let o_file = File::create(format!(
        "{}.{}",
        pack_name,
        if pack_ext == "mrpack" {
            "zip"
        } else {
            "mrpack"
        }
    ))?;
    event!(Level::DEBUG, "Opening {}", pack_filename);

    let mut archive = zip::ZipArchive::new(i_file)?;
    let output = Arc::new(Mutex::new(zip::ZipWriter::new(o_file)));

    let sem_max_concurrent_downloads = Arc::new(Semaphore::new(3));
    let client = Arc::new(
        reqwest::Client::builder()
            .user_agent("Crinfarr/Multipack/indev (dev@crinfarr.io)")
            .default_headers((|| {
                let mut hm = HeaderMap::new();
                let c_key =
                    std::env::var("CURSE_API_KEY").unwrap_or("NO_KEY_SPECIFIED".to_string());
                hm.append("x-api-key", HeaderValue::from_str(&c_key).unwrap());
                hm.append("Accept", HeaderValue::from_str("application/json").unwrap());
                return hm;
            })())
            .build()?,
    );

    for idx in 0..archive.len() {
        let file = archive.name_for_index(idx).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Could not get file at index {idx}"),
        ))?;
        match file {
            "modrinth.index.json" => {
                event!(Level::INFO, "Detected modrinth modpack");
                let _mr_config =
                    serde_json::from_reader::<ZipFile<File>, ModrinthMeta>(archive.by_index(idx)?)?;
                //TODO MODRINTH METADATA TRANSLATION
            }
            "manifest.json" => {
                event!(Level::INFO, "Detected curseforge modpack");
                let curse_config = serde_json::from_reader::<ZipFile<File>, CurseforgeMeta>(
                    archive.by_index(idx)?,
                )?;
                let mut handle_set: JoinSet<()> = JoinSet::new();
                let d_stack: Arc<Mutex<Vec<ModrinthJarMeta>>> = Arc::new(Mutex::new(Vec::new()));
                for file_stats in curse_config.files {
                    let sem_dl = sem_max_concurrent_downloads.clone();
                    let client_ref = client.clone();
                    let fsr = file_stats.clone();
                    let stack_ref = d_stack.clone();
                    let ofile_handle = output.clone();
                    handle_set.spawn(
                    async move {
                        event!(Level::DEBUG, "Waiting on semaphore");
                        let permit = sem_dl.acquire().await.unwrap();
                        //fetch curseforge mod info
                        let r_status = client_ref
                            .get(format!(
                                "https://api.curseforge.com/v1/mods/{}/files/{}",
                                fsr.project_id, fsr.file_id
                            ))
                            .send()
                            .await
                            .unwrap();
                        let r_code = r_status.status();
                        if !r_code.is_success() {
                            event!(Level::WARN, "Curse api req for {}/{} failed: {}", fsr.project_id, fsr.file_id, r_code);
                            return;
                        }
                        //parse mod info response
                        let r_string = match r_status.text().await {
                            Ok(s) => s,
                            Err(e) => {
                                event!(
                                    Level::ERROR,
                                    "Error while fetching {}/{}: {}",
                                    fsr.project_id,
                                    fsr.file_id,
                                    e
                                );
                                return;
                            }
                        };
                        let file_info = match serde_json::from_str::<CurseGetModFileResponse>(&r_string) {
                            Ok(s) => s,
                            Err(e) => {
                                event!(
                                    Level::ERROR,
                                    "Error parsing {}/{}: {}",
                                    fsr.project_id,
                                    fsr.file_id,
                                    e
                                );
                                event!(Level::DEBUG, "Response value: {:?}", &r_string);
                                event!(Level::DEBUG, "Error code: {:?}", r_code);
                                return;
                            }
                        };
                        //find SHA1
                        let file_hash = file_info
                            .data
                            .hashes
                            .iter()
                            .find(|item| item.algo == CurseHashAlgo::Sha1);
                        match file_hash {
                            Some(hashdata) => {
                                //search modrinth by sha1
                                let mr_resp = match client_ref.get(format!("https://api.modrinth.com/v2/version_file/{}?algorithm=sha1",hashdata.value)).send().await {
                                    Ok(v) => v,
                                    Err(e) => {
                                        event!(Level::ERROR, "Failed to fetch modrinth data: {}", e);
                                        return;
                                    }
                                };
                                if !mr_resp.status().is_success() {
                                    event!(Level::WARN, "Mod {} is not available on modrinth, patching directly!", file_info.data.display_name);
                                    event!(Level::DEBUG, "Downloading {}", file_info.data.display_name);
                                    let dl_url = match file_info.data.download_url {
                                        Some(str) => str,
                                        None => {
                                            event!(Level::ERROR, "No download url available for {}", file_info.data.display_name);
                                            return;
                                        }
                                    };
                                    let req = match client_ref.get(dl_url).send().await {
                                        Ok(r) => r,
                                        Err(e) => {
                                            event!(Level::ERROR, "Failed to download {}: {}", file_info.data.display_name, e);
                                            return;
                                        }
                                    };
                                    if !req.status().is_success() {
                                        event!(Level::ERROR, "Failed to download {}: Error {}", file_info.data.display_name, req.status());
                                        return;
                                    }
                                    let content:Vec<u8> = match req.bytes().await {
                                        Ok(b) => b.iter().map(|v| *v).collect(),
                                        Err(e) => {
                                            event!(Level::ERROR, "Failed to parse bytes from body: {}", e);
                                            return;
                                        }
                                    };
                                    event!(Level::DEBUG, "Waiting for output zip lock");
                                    let mut o_lock = ofile_handle.lock().await;
                                    let _ = o_lock.start_file(format!("overrides/mods/{}", file_info.data.file_name), SimpleFileOptions::default());
                                    let _ = o_lock.write(&content);
                                    return;
                                }
                                let r_body = match mr_resp.text().await {
                                    Ok(t) => t,
                                    Err(e) => {
                                        event!(Level::ERROR, "Failed to get body from modrinth request: {}", e);
                                        return;
                                    }
                                };
                                let mr_val = match serde_json::from_str::<ModrinthGetVersionFromHashResponse>(&r_body) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        event!(Level::ERROR, "Failed to deserialize modrinth request: {}", e);
                                        event!(Level::DEBUG, "Body: {}", r_body);
                                        return;
                                    }
                                };
                                event!(Level::INFO, "{} found on modrinth at {}", file_info.data.display_name, mr_val.files[0].url);
                                stack_ref.lock().await.push(ModrinthJarMeta {
                                    path: format!("mods/{}", mr_val.files[0].filename),
                                    hashes: ModrinthHashInfo {
                                        sha512: mr_val.files[0].hashes.sha512.clone(),
                                        sha1: mr_val.files[0].hashes.sha1.clone()
                                    },
                                    env: ModrinthEnvironmentRequirement {
                                        client: "required".to_string(),
                                        server:"required".to_string()
                                    },
                                    downloads: vec![mr_val.files[0].url.clone()],
                                    file_size: mr_val.files[0].size
                                });
                            },
                            None => {
                                event!(Level::ERROR, "No SHA1 hash available from curseforge");
                                return;
                            }
                        }
                        drop(permit);
                    }
                    .instrument(span!(
                        Level::INFO,
                        "Download Thread",
                        File = format!("{}/{}", file_stats.project_id, file_stats.file_id)
                    )),
                );
                }
                event!(Level::DEBUG, "All threads spawned");
                handle_set.join_all().await;
                let s_lock = d_stack.lock().await;
                let mod_conf: Vec<ModrinthJarMeta> = s_lock.iter().map(|v| v.clone()).collect();
                let finished_conf = ModrinthMeta {
                    dependencies: (|| {
                        let mut depmap: BTreeMap<String, String> = BTreeMap::new();
                        curse_config
                            .minecraft
                            .mod_loaders
                            .iter()
                            .for_each(|loader| {
                                let loader_spec: Vec<String> = loader
                                    .id
                                    .split("-")
                                    .take(2)
                                    .map(|s| s.to_string())
                                    .collect();
                                depmap.insert(loader_spec[0].clone(), loader_spec[1].clone());
                            });
                        depmap.insert("minecraft".to_string(), curse_config.minecraft.version);
                        return depmap;
                    })(),
                    files: mod_conf,
                    format_version: 1,
                    game: "minecraft".to_string(),
                    name: curse_config.name,
                    summary: "".to_string(),
                    version_id: "1.0.0".to_string(),
                };
                let mut o_lock = output.lock().await;
                o_lock.start_file("modrinth.index.json", SimpleFileOptions::default())?;
                o_lock.write(&serde_json::to_vec(&finished_conf)?)?;
            }
            f => {
                event!(Level::DEBUG, "Copying {}", f);
                let mut o_lock = output.lock().await;
                o_lock.start_file(f, SimpleFileOptions::default())?;
                let f_bytes: Vec<u8> = archive.by_index(idx)?.bytes().map(|b| b.unwrap()).collect();
                o_lock.write(&f_bytes)?;
            }
        }
    }
    drop(output);//HACK THIS IS REALLY FUCKING STUPID DO NOT DO THIS
    Ok(())
}
