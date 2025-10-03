use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
//FILE CONTENT
/**
 * Pack metadata file as contained in the modpack file
 */
#[derive(Deserialize, Serialize)]
#[allow(unused)]
pub struct PackMeta {
    pub game: String,
    #[serde(rename = "formatVersion")]
    pub format_version: u8,
    #[serde(rename = "versionId")]
    pub version_id: String,
    pub name: String,
    pub summary: String,
    pub files: Vec<PackModDescription>,
    pub dependencies: BTreeMap<String, String>,
}
/**
 * Metadata for a mod, generally in a [Vec] in the [PackMeta]
 */
#[derive(Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct PackModDescription {
    pub path: String,
    pub hashes: HashInfo,
    pub env: ModrinthEnvironmentRequirement,
    pub downloads: Vec<String>,
    #[serde(rename = "fileSize")]
    pub file_size: u32,
}
/**
 * Hashes for a [PackModDescription]
 */
#[derive(Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct HashInfo {
    pub sha512:String,
    pub sha1:String
}
/**
 * Requirement state on client and server
 */
#[derive(Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct ModrinthEnvironmentRequirement {
    pub client: String,
    pub server: String,
}

//API RESPONSES
/**
 * Response to modrinth api @ GET /v2/version_file/{SHA1_HASH}
 */
#[derive(Deserialize)]
#[allow(unused)]
pub struct VersionFileResponse {
    pub name:String,
    pub version_number:String,
    pub changelog:Option<String>,
    pub dependencies:Vec<DependencyInfo>,
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
/**
 * Project information about a dep for this mod
 */
#[derive(Deserialize)]
#[allow(unused)]
pub struct DependencyInfo {
    pub version_id:Option<String>,
    pub project_id:Option<String>,
    pub file_name:Option<String>,
    pub dependency_type:String
}
/**
 * Info about this mod version
 */
#[derive(Deserialize)]
#[allow(unused)]
pub struct ModrinthFileInfo {
    pub hashes:HashInfo,
    pub url:String,
    pub filename:String,
    pub primary:bool,
    pub size:u32,
    pub file_type:Option<String>
}
