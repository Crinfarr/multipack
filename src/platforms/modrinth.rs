use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[allow(unused)]
pub struct ModrinthMeta {
    pub game: String,
    #[serde(rename = "formatVersion")]
    pub format_version: u8,
    #[serde(rename = "versionId")]
    pub version_id: String,
    pub name: String,
    pub summary: String,
    pub files: Vec<ModrinthJarMeta>,
    pub dependencies: BTreeMap<String, String>,
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct ModrinthGetVersionFromHashResponse {
    pub name:String,
    pub version_number:String,
    pub changelog:Option<String>,
    pub dependencies:Vec<ModrinthDependencyInfo>,
    pub game_versions:Vec<String>,
    pub version_type:String,
    pub loaders:Vec<String>,
    pub featured:bool,
    pub status:String,
    pub requested_status:Option<String>,
    pub id:String,
    pub project_id:String,
    pub author_id:String,
    pub date_published:String,
    pub downloads:u32,
    pub changelog_url:Option<String>,
    pub files:Vec<ModrinthFileInfo>
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct ModrinthDependencyInfo {
    pub version_id:Option<String>,
    pub project_id:Option<String>,
    pub file_name:Option<String>,
    pub dependency_type:String
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct ModrinthFileInfo {
    pub hashes:ModrinthHashInfo,
    pub url:String,
    pub filename:String,
    pub primary:bool,
    pub size:u32,
    pub file_type:Option<String>
}
#[derive(Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct ModrinthHashInfo {
    pub sha512:String,
    pub sha1:String
}
#[derive(Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct ModrinthJarMeta {
    pub path: String,
    pub hashes: ModrinthHashInfo,
    pub env: ModrinthEnvironmentRequirement,
    pub downloads: Vec<String>,
    #[serde(rename = "fileSize")]
    pub file_size: u32,
}
#[derive(Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct ModrinthEnvironmentRequirement {
    pub client: String,
    pub server: String,
}
