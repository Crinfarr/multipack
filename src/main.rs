mod pack_meta;

use std::{
    fs::File,
    io::{self}, sync::Arc,
};

use tokio::{sync::{Mutex, Semaphore}, task::JoinSet};
use tracing::{event, span, Instrument, Level};
use zip::{read::ZipFile, result::ZipResult, ZipArchive};

use crate::pack_meta::{CurseGetModResponse, CurseforgeMeta, ModrinthMeta};
enum PackMeta {
    Modrinth(ModrinthMeta),
    Curse(CurseforgeMeta),
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let mut args = std::env::args();
    args.next();//HACK skip the exe you dumb motherfucker i hate you (me)
    let pack_filename = args.next().ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "No file specified",
    ))?;
    let i_file = File::open(&pack_filename)?;
    event!(Level::DEBUG, "Opening {}", pack_filename);

    let mut archive = zip::ZipArchive::new(i_file)?;
    
    let sem_max_concurrent_downloads = Arc::new(Semaphore::new(100));
    let client = Arc::new(reqwest::Client::builder()
        .user_agent("io.crinfarr.Multipack")
        .build()?);
    
    for idx in 0..archive.len() {
        let file = archive.name_for_index(idx).ok_or(
            io::Error::new(io::ErrorKind::InvalidData, format!("Could not get file at index {idx}"))
        )?;
        if file == "modrinth.index.json" {
            let mr_config = serde_json::from_reader::<ZipFile<File>, ModrinthMeta>(archive.by_index(idx)?)?;
            //TODO MODRINTH METADATA TRANSLATION
        } else if file == "manifest.json" {
            let curse_config = serde_json::from_reader::<ZipFile<File>, CurseforgeMeta>(archive.by_index(idx)?)?;
            let mut handle_set:JoinSet<()> = JoinSet::new();
            for file_stats in curse_config.files {
                let sem_dl = sem_max_concurrent_downloads.clone();
                let client_ref = client.clone();
                let fsr = file_stats.clone();
                handle_set.spawn(async move {
                    event!(Level::DEBUG, "Waiting on semaphore");
                    let permit = sem_dl.acquire().await.unwrap();
                    let res = client_ref.get(format!("https://api.curseforge.com/v1/mods/{}", fsr.file_id)).send().await.unwrap();
                    let modinfo = serde_json::from_str::<CurseGetModResponse>(&res.text().await.unwrap()).unwrap();
                    drop(permit);
                    
                }.instrument(span!(Level::INFO, "Download Thread", FileID = file_stats.file_id)));
            }
        }
    }

    Ok(())
}
