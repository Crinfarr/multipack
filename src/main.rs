use color_eyre::Result;
use std::{
    fs::File,
    io::{Bytes, Read, Write},
    sync::Arc,
};
use tokio::{sync::Mutex, task::{JoinHandle, JoinSet}};
use tracing::{Instrument, Level, event, span};
use zip::{read::ZipFile, write::SimpleFileOptions, ZipWriter};

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
    color_eyre::install()?;
    dotenv::dotenv()?;
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
        .unwrap();

    let out_writer = match write_type {
        OutputFormat::CURSEFORGE => {
            todo!("Curseforge is not yet supported");
        }
        OutputFormat::TECHNICPACK => {
            todo!("Technic is not yet supported");
        }
        OutputFormat::MODRINTH => {
            Arc::new(Mutex::new(ZipWriter::new(File::create("pack.mrpack")?)))
        }
        OutputFormat::OTHER(s) => todo!("Format {} not supported", s),
    };

    for index in 0..pack_reader.len() {
        let file = pack_reader.by_index(index)?;
        if file.is_dir() {
            out_writer
                .lock()
                .await
                .add_directory(file.name(), SimpleFileOptions::default())?;
            continue;
        }
        let f_name = file.name().to_string();
        let content = file.bytes();
        let mut handles = JoinSet::new();
        match f_name.as_str() {
            "index.json" => {
                let config = serde_json::from_reader::<File, platforms::curse::CurseforgeMeta>(content);
            }

            other => {
                let out_ref = out_writer.clone();
                let filename  = other.to_string();
                let f_content:Vec<u8> = content.map(|f| f.unwrap().clone()).collect();
                handles.spawn(
                    async move {
                        event!(Level::DEBUG, "Waiting on write mutex");
                        let mut output = out_ref.lock().await;
                        let mut start_at: u64 = 0;
                        output.start_file(filename, SimpleFileOptions::default()).unwrap();
                        loop {
                            let written = output.write(&f_content[(start_at as usize)..]).unwrap();
                            if written == 0 {
                                break;
                            } else {
                                start_at += written as u64;
                            }
                        }
                    }
                    .instrument(span!(
                        Level::INFO,
                        "Writer Thread",
                        file = other
                    )),
                );
            }
        }
        handles.join_all().await;
    }

    Ok(())
}
