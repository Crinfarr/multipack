use crate::platforms::{mr::{PackModDescription, VersionFileResponse}};
use super::ModInfo;
impl From<PackModDescription> for ModInfo<PackModDescription, VersionFileResponse> {
    fn from(value: PackModDescription) -> Self {
        Self {
            config: value,
            resolved_info: None,
            client: reqwest::Client::default(),
            file_name: None,
            resolved: false,
            sha1: None
        }
    }
}