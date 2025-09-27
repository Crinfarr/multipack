mod pack_meta;

use std::{
    fs::File,
    io::{self},
};

use tokio::sync::Semaphore;
use tracing::{Level, event};
use zip::{read::ZipFile, result::ZipResult};

use crate::pack_meta::{CurseforgeMeta, ModrinthMeta};
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
    for idx in 0..archive.len() {
        let file = archive.name_for_index(idx).ok_or(
            io::Error::new(io::ErrorKind::InvalidData, format!("Could not get file at index {idx}"))
        )?;
        if file == "modrinth.index.json" {
            let mr_config = serde_json::from_reader::<ZipFile<File>, ModrinthMeta>(archive.by_index(idx)?)?;
            //TODO MODRINTH METADATA TRANSLATION
        } else if file == "manifest.json" {
            let curse_config = serde_json::from_reader::<ZipFile<File>, CurseforgeMeta>(archive.by_index(idx)?)?;
            //TODO CURSE METADATA TRANSLATION
        }
    }
    let sem_max_concurrent_downloads = Semaphore::new(100);
    let client = reqwest::Client::builder()
        .user_agent("io.crinfarr.Multipack")
        .build()?;

    Ok(())
}
