use serde::Deserialize;

#[derive(Deserialize)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeMeta {
    pub minecraft: CurseMinecraftMetadata,
    pub manifest_type: String,
    pub manifest_version: u8,
    pub name: String,
    pub version: String,
    pub author: String,
    pub files: Vec<CurseFileDescription>,
    pub overrides: String,
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct CurseGetModResponse {
    pub data: CurseMod,
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct CurseGetModFileResponse {
    pub data: CurseFile,
}
#[derive(Deserialize)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct CurseMinecraftMetadata {
    pub version: String,
    pub mod_loaders: Vec<CurseModLoaderMeta>,
}
#[derive(Deserialize)]
#[allow(unused)]
pub struct CurseModLoaderMeta {
    pub id: String,
    pub primary: bool,
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct CurseFileDescription {
    #[serde(rename = "projectID")]
    pub project_id: u32,
    #[serde(rename = "fileID")]
    pub file_id: u32,
    pub required: bool,
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct CurseMod {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub links: CurseModLinks,
    pub summary: String,
    pub status: CurseModStatus,
    pub download_count: i64,
    pub is_featured: bool,
    pub primary_category_id: u32,
    pub categories: Vec<CurseCategory>,
    pub class_id: Option<u32>,
    pub authors: Vec<CurseModAuthor>,
    pub logo: CurseModAsset,
    pub screenshots: CurseModAsset,
    pub main_file_id: u32,
    pub latest_files: Vec<CurseFile>,
    pub latest_file_indexes: Vec<CurseFileIndex>,
    pub latest_early_access_files_indexes:Vec<CurseFileIndex>,
    pub date_created:String,
    pub date_modified:String,
    pub date_released:String,
    pub allow_mod_distribution:Option<bool>,
    pub game_popularity_link:u32,
    pub is_available:bool,
    pub thumbs_up_count:bool,
    pub rating:Option<f64>,
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct CurseModLinks {
    pub website_url: String,
    pub wiki_url: String,
    pub issues_url: String,
    pub source_url: String,
}
#[derive(Clone)]
pub enum CurseModStatus {
    New,
    ChangesRequired,
    UnderSoftReview,
    Approved,
    Rejected,
    ChangesMade,
    Inactive,
    Abandoned,
    Deleted,
    UnderReview,
}
impl<'de> Deserialize<'de> for CurseModStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match u32::deserialize(deserializer) {
            Ok(i) => {
                if i <= 10 && i >= 1 {
                    Ok([
                        Self::New,
                        Self::ChangesRequired,
                        Self::UnderSoftReview,
                        Self::Approved,
                        Self::Rejected,
                        Self::ChangesMade,
                        Self::Inactive,
                        Self::Abandoned,
                        Self::Deleted,
                        Self::UnderReview,
                    ][i as usize - 1]
                        .clone())
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(i as u64),
                        &"a number [1,10]",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct CurseCategory {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub icon_url: String,
    pub date_modified: String,
    pub is_class: Option<bool>,
    pub class_id: Option<u32>,
    pub parent_category_id: Option<u32>,
    pub display_index: Option<u32>,
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct CurseModAuthor {
    pub id: u32,
    pub name: String,
    pub url: String,
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct CurseModAsset {
    pub id: u32,
    pub mod_id: u32,
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,
    pub url: String,
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct CurseFile {
    pub id: u32,
    pub game_id: u32,
    pub mod_id: u32,
    pub is_available: bool,
    pub display_name: String,
    pub file_name: String,
    pub release_type: CurseFileReleaseType,
    pub file_status: CurseFileStatus,
    pub hashes: Vec<CurseFileHash>,
    pub file_date: String,
    pub file_length: i64,
    pub download_count: i64,
    pub file_size_on_disk: Option<i64>,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
    pub sortable_game_versions: Vec<CurseSortableGameVersions>,
    pub dependencies: Vec<CurseFileDependency>,
    pub expose_as_alternative: Option<bool>,
    pub parent_project_file_id: Option<u32>,
    pub alternate_file_id: Option<u32>,
    pub is_server_pack: Option<bool>,
    pub is_early_access_content: Option<bool>,
    pub early_access_end_date: Option<String>,
    pub file_fingerprint: i64,
    pub modules: Vec<CurseFileModule>,
}
#[derive(Clone)]
pub enum CurseFileReleaseType {
    Release,
    Beta,
    Alpha,
}
impl<'de> Deserialize<'de> for CurseFileReleaseType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match u32::deserialize(deserializer) {
            Ok(i) => {
                if i <= 3 && i >= 1 {
                    Ok([Self::Release, Self::Beta, Self::Alpha][i as usize - 1].clone())
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(i as u64),
                        &"a number [1,3]",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}
#[derive(Clone)]
pub enum CurseFileStatus {
    Processing,
    ChangesRequired,
    UnderReview,
    Approved,
    Rejected,
    MalwareDetected,
    Deleted,
    Archived,
    Testing,
    Released,
    ReadyForReview,
    Deprecated,
    Baking,
    AwaitingPublishing,
    FailedPublishing,
    Cooking,
    Cooked,
    UnderManualReview,
    ScanningForMalware,
    ProcessingFile,
    PendingRelease,
    ReadyForCooking,
    PostProcessing,
}
impl<'de> Deserialize<'de> for CurseFileStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match u32::deserialize(deserializer) {
            Ok(i) => {
                if i <= 23 && i >= 1 {
                    Ok([
                        Self::Processing,
                        Self::ChangesRequired,
                        Self::UnderReview,
                        Self::Approved,
                        Self::Rejected,
                        Self::MalwareDetected,
                        Self::Deleted,
                        Self::Archived,
                        Self::Testing,
                        Self::Released,
                        Self::ReadyForReview,
                        Self::Deprecated,
                        Self::Baking,
                        Self::AwaitingPublishing,
                        Self::FailedPublishing,
                        Self::Cooking,
                        Self::Cooked,
                        Self::UnderManualReview,
                        Self::ScanningForMalware,
                        Self::ProcessingFile,
                        Self::PendingRelease,
                        Self::ReadyForCooking,
                        Self::PostProcessing,
                    ][i as usize - 1]
                        .clone())
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(i as u64),
                        &"a number [1,23]",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct CurseFileHash {
    pub value: String,
    pub algo: CurseHashAlgo,
}
#[derive(Clone, PartialEq)]
pub enum CurseHashAlgo {
    Sha1,
    Md5,
}
impl<'de> Deserialize<'de> for CurseHashAlgo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match u32::deserialize(deserializer) {
            Ok(i) => {
                if i >= 1 && i <= 2 {
                    Ok([Self::Sha1, Self::Md5][i as usize - 1].clone())
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(i as u64),
                        &"a number [1,2]",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct CurseSortableGameVersions {
    pub game_version_name: String,
    pub game_version_padded: String,
    pub game_version: String,
    pub game_version_release_date: String,
    pub game_version_type_id: Option<u32>,
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct CurseFileDependency {
    pub mod_id: u32,
    pub relation_type: CurseFileRelationType,
}
#[derive(Clone)]
pub enum CurseFileRelationType {
    EmbeddedLibrary,
    OptionalDependency,
    RequiredDependency,
    Tool,
    Incomplete,
    Include,
}
impl<'de> Deserialize<'de> for CurseFileRelationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match u32::deserialize(deserializer) {
            Ok(i) => {
                if i <= 6 && i >= 1 {
                    Ok([
                        Self::EmbeddedLibrary,
                        Self::OptionalDependency,
                        Self::RequiredDependency,
                        Self::Tool,
                        Self::Incomplete,
                        Self::Include,
                    ][i as usize - 1]
                        .clone())
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(i as u64),
                        &"a number [1,6]",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct CurseFileModule {
    pub name: String,
    pub fingerprint: i64,
}
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct CurseFileIndex {
    pub game_version: String,
    pub file_id: u32,
    pub filename: String,
    pub release_type: CurseFileReleaseType,
    pub game_version_type_id: Option<u32>,
    pub mod_loader: CurseModLoaderType,
}
#[derive(Clone)]
pub enum CurseModLoaderType {
    Any,
    Forge,
    Cauldron,
    LiteLoader,
    Fabric,
    Quilt,
    NeoForge,
}
impl<'de> Deserialize<'de> for CurseModLoaderType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match u32::deserialize(deserializer) {
            Ok(i) => {
                if i <= 6 {
                    Ok([
                        Self::Any,
                        Self::Forge,
                        Self::Cauldron,
                        Self::LiteLoader,
                        Self::Fabric,
                        Self::Quilt,
                        Self::NeoForge,
                    ][i as usize]
                        .clone())
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(i as u64),
                        &"a number [0,6]",
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
}
