use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[allow(unused)]
pub struct ModrinthMeta {
    game:String,
    #[serde(rename = "formatVersion")]
    format_version:u8,
    #[serde(rename = "versionId")]
    version_id:String,
    name:String,
    summary:String,
    files:Vec<ModrinthJarMeta>,
    dependencies:BTreeMap<String, String>
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct ModrinthJarMeta {
    path:String,
    hashes: ModrinthJarFileHashes,
    env: ModrinthEnvironmentRequirement,
    downloads:Vec<String>,
    #[serde(rename = "fileSize")]
    file_size:u32,
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct ModrinthJarFileHashes {
    sha512:String,
    sha1:String,
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct ModrinthEnvironmentRequirement {
    client:String,
    server:String
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct CurseforgeMeta {
    minecraft:CurseMinecraftMetadata,
    #[serde(rename = "manifestType")]
    manifest_type:String,
    #[serde(rename = "manifestVersion")]
    manifest_version:u8,
    name:String,
    version:String,
    author:String,
    files:Vec<CurseFileDescription>,
    overrides:String
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct CurseMinecraftMetadata {
    version:String,
    #[serde(rename = "modLoaders")]
    mod_loaders:Vec<CurseModLoaderMeta>,
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct CurseModLoaderMeta {
    id:String,
    primary:bool
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct CurseFileDescription {
    #[serde(rename = "projectID")]
    project_id:String,
    #[serde(rename = "fileID")]
    file_id:String,
    required:bool
}