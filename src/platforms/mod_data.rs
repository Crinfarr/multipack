pub mod curse_mod_data;
pub mod modrinth_mod_data;

use std::{error::Error, fmt::Display};

use crate::platforms::{curse::CurseDependency, mr::ModrinthDependency};

#[derive(Debug)]
pub struct FetchError(String);
impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for FetchError {}

#[derive(Default, Debug)]
pub struct ModInfo<ConfType, ResolvableType, DepType> {
    pub(super) config: ConfType,
    pub(super) resolved_info: Option<ResolvableType>,
    pub deps: Option<Vec<DepType>>,
    pub sha1: Option<String>,
    pub file_name: Option<String>,
    pub(in crate::platforms::mod_data) client: reqwest::Client, //reqwest::Client already uses Arc internally for clones (i am brain damaged)
    pub(super) resolved: bool,
}
