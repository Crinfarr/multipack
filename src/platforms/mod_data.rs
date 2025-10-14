pub mod curse_mod_data;
pub mod modrinth_mod_data;

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct FetchError(String);
impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for FetchError {}

#[derive(Default, Debug)]
pub struct ModInfo<T, U> {
    pub(super) config: T,
    pub(super) resolved_info: Option<U>,
    pub deps: Option<Vec<DependencyInfo>>,
    pub sha1: Option<String>,
    pub file_name: Option<String>,
    pub(in crate::platforms::mod_data) client: reqwest::Client, //reqwest::Client already uses Arc internally for clones (i am brain damaged)
    pub(super) resolved: bool,
}
pub trait DepResolve {
    async fn resolve_deps(self) -> Self;
}
#[derive(Debug)]
pub struct DependencyInfo {
    curse_project_id: Option<u32>,
    sha1: Option<String>,
}
