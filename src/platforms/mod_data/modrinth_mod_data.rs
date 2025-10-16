use super::ModInfo;
use crate::platforms::mr::{PackModDescription, VersionFileResponse};
impl From<PackModDescription> for ModInfo<PackModDescription, VersionFileResponse, ModrinthDependency> {
    fn from(value: PackModDescription) -> Self {
        Self {
            config: value,
            resolved_info: None,
            deps: None,
            client: reqwest::Client::default(),
            file_name: None,
            resolved: false,
            sha1: None,
        }
    }
}

#[derive(Debug)]
pub struct ModrinthDependency(String);
