use color_eyre::{Result, eyre::Context};
use std::{
    fs::File,
    io::{Read, Write},
    sync::Arc,
};
use tokio::{sync::Mutex, task::JoinSet, time::Instant};
use tracing::{Instrument, Level, event, span};
use zip::{ZipWriter, read::ZipFile, write::SimpleFileOptions};

mod mod_data;
mod platforms;

enum OutputFormat {
    MODRINTH,
    CURSEFORGE,
    TECHNICPACK,
    OTHER(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().wrap_err("Err while initializing color_eyre")?;
    dotenv::dotenv().wrap_err("Error while parsing .env")?;
    tracing_subscriber::fmt()
        .with_max_level(
            match std::env::var("LOGLEVEL")
                .unwrap_or("undefined".to_string())
                .as_str()
            {
                "TRACE" => Level::TRACE,
                "DEBUG" => Level::DEBUG,
                "INFO" => Level::INFO,
                "WARN" => Level::WARN,
                "ERROR" => Level::ERROR,
                _ => Level::INFO,
            },
        )
        .init();
    let st = Instant::now();
    let write_type = match std::env::var("OUTPUT_FORMAT")
        .unwrap_or("UNSPECIFIED".to_string())
        .as_str()
    {
        "MODRINTH" => OutputFormat::MODRINTH,
        "CURSE" => OutputFormat::CURSEFORGE,
        "TECHNIC" => OutputFormat::TECHNICPACK,
        other => OutputFormat::OTHER(other.to_string()),
    };

    let mut args = std::env::args();
    args.next(); //skip executable
    let (pack_path, pack_ext, mut pack_reader) = args
        .next()
        .map(|s| {
            let mut splits: Vec<&str> = s.split(".").collect();
            let ext = splits.pop().unwrap();
            let path = splits.join(".");
            (
                path.to_string(),
                ext.to_string(),
                zip::ZipArchive::new(File::open(s).unwrap()).unwrap(),
            )
        })
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to open input",
        ))?;

    let out_writer = match write_type {
        OutputFormat::CURSEFORGE => {
            todo!("Curseforge is not yet supported");
        }
        OutputFormat::TECHNICPACK => {
            todo!("Technic is not yet supported");
        }
        OutputFormat::MODRINTH => Arc::new(Mutex::new(ZipWriter::new(
            File::create("pack.mrpack").wrap_err("Err while attempting to create output zip")?,
        ))),
        OutputFormat::OTHER(s) => todo!("Format {} not supported", s),
    };
    let mut handles = JoinSet::new();

    for index in 0..pack_reader.len() {
        let file = pack_reader
            .by_index(index)
            .wrap_err("Err while reading input zip")?;
        if file.is_dir() {
            out_writer
                .lock()
                .await
                .add_directory(file.name(), SimpleFileOptions::default())
                .wrap_err("Err while copying directory")?;
            continue;
        }
        let f_name = file.name().to_string();
        match f_name.as_str() {
            "manifest.json" => {
                let file = serde_json::from_str::<platforms::curse::PackMeta>(
                    &file.bytes()
                        .map(|b| b.unwrap() as char)
                        .collect::<String>(),
                ).wrap_err("Err while loading curseforge manifest")?;
                // let _config = serde_json::from_reader::<ZipFile<File>, platforms::curse::CurseforgeMeta>(file).wrap_err("Error while parsing metadata")?;
                event!(Level::WARN, "CONFIG PARSING IS NYI");
                event!(Level::DEBUG, "{:#?}", file);
                continue;
            }

            other => {
                let out_ref = out_writer.clone();
                let filename = other.to_string();
                event!(Level::TRACE, "Vectorizing {}", filename);
                let f_bytes: Vec<u8> = file.bytes().map(|f| f.unwrap().clone()).collect();
                event!(Level::DEBUG, "Spawning write thread for {}", filename);
                handles.spawn(
                    async move {
                        event!(Level::DEBUG, "Waiting on write mutex");
                        let mut output = out_ref.lock().await;
                        event!(Level::TRACE, "Got permit");
                        let mut start_at: u64 = 0;
                        output
                            .start_file(filename, SimpleFileOptions::default())
                            .unwrap();
                        loop {
                            let written = output.write(&f_bytes[(start_at as usize)..]).unwrap();
                            if written == 0 {
                                break;
                            } else {
                                start_at += written as u64;
                            }
                        }
                        event!(Level::DEBUG, "Wrote {:?}b", start_at);
                    }
                    .instrument(span!(
                        Level::INFO,
                        "Writer Thread",
                        file = other
                    )),
                );
            }
        }
    }

    event!(Level::INFO, "Waiting for writer threads");
    handles.join_all().await;

    event!(
        Level::INFO,
        "Done in {:?}",
        Instant::now().duration_since(st)
    );
    Ok(())
}
