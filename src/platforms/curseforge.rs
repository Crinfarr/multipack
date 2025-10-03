use serde::Deserialize;

//FILE CONTENT
/**
 * Pack metadata file as contained in the modpack zip
 */
#[derive(Deserialize, Debug)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct PackMeta {
    pub minecraft: PackMinecraftMetadata,
    pub manifest_type: String,
    pub manifest_version: u8,
    pub name: String,
    pub version: String,
    pub author: String,
    pub files: Vec<PackModDescription>,
    pub overrides: String,
}
/**
 * Metadata for the Minecraft version requirement of a modpack
 */
#[derive(Deserialize, Debug)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct PackMinecraftMetadata {
    pub version: String,
    pub mod_loaders: Vec<PackModLoaderMetadata>,
}
/**
 * Metadata for the Minecraft modloaders (quilt, neo/forge, etc) required by a modpack
 */
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct PackModLoaderMetadata {
    pub id: String,
    pub primary: bool,
}
/**
 * Metadata for a mod, generally in a [Vec] in the [PackMeta]
 */
#[derive(Deserialize, Clone, Debug)]
#[allow(unused)]
pub struct PackModDescription {
    #[serde(rename = "projectID")]
    pub project_id: u32,
    #[serde(rename = "fileID")]
    pub file_id: u32,
    pub required: bool,
}
//API CONTENT
/**
 * Response to curse api @ GET /v1/mods/{ID}
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct ModLookupResponse {
    pub data: APIModData,
}
/**
 * Response to curse api @ GET /v1/mods/{ID}/files/{ID}
 */
#[derive(Deserialize)]
#[allow(unused)]
pub struct FileLookupResponse {
    pub data: APIFile,
}
/**
 * Data about a mod as retrieved by the curse api
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct APIModData {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub links: ModLinks,
    pub summary: String,
    pub status: ModStatus,
    pub download_count: i64,
    pub is_featured: bool,
    pub primary_category_id: u32,
    pub categories: Vec<ModCategory>,
    pub class_id: Option<u32>,
    pub authors: Vec<ModAuthor>,
    pub logo: AssetMeta,
    pub screenshots: AssetMeta,
    pub main_file_id: u32,
    pub latest_files: Vec<APIFile>,
    pub latest_file_indexes: Vec<FileIndex>,
    pub latest_early_access_files_indexes: Vec<FileIndex>,
    pub date_created: String,
    pub date_modified: String,
    pub date_released: String,
    pub allow_mod_distribution: Option<bool>,
    pub game_popularity_link: u32,
    pub is_available: bool,
    pub thumbs_up_count: bool,
    pub rating: Option<f64>,
}
/**
 * Links related to [APIModData]
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct ModLinks {
    pub website_url: String,
    pub wiki_url: String,
    pub issues_url: String,
    pub source_url: String,
}
/**
 * The status of a mod as listed in the curseforge api
 */
#[derive(Clone)]
pub enum ModStatus {
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
/**
 * The category that this [APIModData] is categorized into
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct ModCategory {
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
/**
 * Details about the author of this [APIModData]
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct ModAuthor {
    pub id: u32,
    pub name: String,
    pub url: String,
}
/**
 * Metadata for an asset as attached to this [APIModData]
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct AssetMeta {
    pub id: u32,
    pub mod_id: u32,
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,
    pub url: String,
}
/**
 * Data about a file as retrieved from the curse API
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct APIFile {
    pub id: u32,
    pub game_id: u32,
    pub mod_id: u32,
    pub is_available: bool,
    pub display_name: String,
    pub file_name: String,
    pub release_type: ReleaseType,
    pub file_status: FileStatus,
    pub hashes: Vec<FileHashData>,
    pub file_date: String,
    pub file_length: i64,
    pub download_count: i64,
    pub file_size_on_disk: Option<i64>,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
    pub sortable_game_versions: Vec<SortableGameVersion>,
    pub dependencies: Vec<FileDependency>,
    pub expose_as_alternative: Option<bool>,
    pub parent_project_file_id: Option<u32>,
    pub alternate_file_id: Option<u32>,
    pub is_server_pack: Option<bool>,
    pub is_early_access_content: Option<bool>,
    pub early_access_end_date: Option<String>,
    pub file_fingerprint: i64,
    pub modules: Vec<Module>,
}
/**
 * The release state (Alpha/beta or Release)
 */
#[derive(Clone)]
pub enum ReleaseType {
    Release,
    Beta,
    Alpha,
}
/**
 * Status of this file
 */
#[derive(Clone)]
pub enum FileStatus {
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
/**
 * File hash w/ algorithm specifier
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct FileHashData {
    pub value: String,
    pub algo: HashAlgo,
}
/**
 * Hash algorithm of this [FileHashData]
 */
#[derive(Clone, PartialEq)]
pub enum HashAlgo {
    Sha1,
    Md5,
}
/**
 * Game version specs but easier to sort
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct SortableGameVersion {
    pub game_version_name: String,
    pub game_version_padded: String,
    pub game_version: String,
    pub game_version_release_date: String,
    pub game_version_type_id: Option<u32>,
}
/**
 * Dependencies of this [APIFile]
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct FileDependency {
    pub mod_id: u32,
    pub relation_type: RelationType,
}
/**
 * Way this [FileDependency] is related
 */
#[derive(Clone)]
pub enum RelationType {
    EmbeddedLibrary,
    OptionalDependency,
    RequiredDependency,
    Tool,
    Incomplete,
    Include,
}
/**
 * Curse module related to this file
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
pub struct Module {
    pub name: String,
    pub fingerprint: i64,
}
/**
 * Reference to a file
 */
#[derive(Deserialize, Clone)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
pub struct FileIndex {
    pub game_version: String,
    pub file_id: u32,
    pub filename: String,
    pub release_type: ReleaseType,
    pub game_version_type_id: Option<u32>,
    pub mod_loader: CurseModLoaderType,
}
/**
 * Modloader required for this file
 */
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

//enum deserializer impls
// * * * * * * * //
impl<'de> Deserialize<'de> for ModStatus {
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
impl<'de> Deserialize<'de> for RelationType {
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
impl<'de> Deserialize<'de> for HashAlgo {
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
impl<'de> Deserialize<'de> for FileStatus {
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
impl<'de> Deserialize<'de> for ReleaseType {
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