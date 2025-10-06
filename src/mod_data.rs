use std::{error::Error, fmt::Display, sync::Arc};

use color_eyre::eyre::{Context, Result};

use crate::platforms::{
    curse::{HashAlgo as CurseHashAlgo, PackModDescription},
    mr::ModrinthFileInfo,
};

#[derive(Default)]
enum WrappedModInfo {
    CurseMod(crate::platforms::curse::PackModDescription),
    ModrinthMod(crate::platforms::mr::ModrinthFileInfo),
    #[default]
    Undefined,
}

#[derive(Debug)]
pub struct FetchError(String);
impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for FetchError {}

#[derive(Default)]
pub struct ModInfo {
    config: WrappedModInfo,
    sha1: Option<String>,
    name: Option<String>,
    client: reqwest::Client,//reqwest::Client already uses Arc internally for clones (i am brain damaged)
    resolved:bool,
}
impl ModInfo {
    async fn resolve_remotes(&mut self) -> Result<()> {
        match &self.config {
            WrappedModInfo::CurseMod(desc) => {
                let resp = self
                    .client
                    .get(format!(
                        "https://api.curseforge.com/v1/mods/{}/files/{}",
                        desc.project_id, desc.file_id
                    ))
                    .send()
                    .await?;
                if !resp.status().is_success() {
                    return Err(FetchError(format!(
                        "Could not resolve mod: http err {}",
                        resp.status()
                    ))
                    .into());
                }
                let info = serde_json::from_str::<crate::platforms::curse::FileLookupResponse>(
                    resp.text().await?.as_str(),
                )?;
                let hash = info
                    .data
                    .hashes
                    .iter()
                    .find(|h| h.algo == CurseHashAlgo::Sha1)
                    .take()
                    .ok_or("No SHA1 hash in api response")
                    .map_err(|e| FetchError(e.to_string()))
                    .map(|f| f.value.clone())?;
                self.sha1 = Some(hash);
                let name = info.data.display_name;
            }
            WrappedModInfo::ModrinthMod(desc) => {}
            WrappedModInfo::Undefined => self.sha1 = None,
        };

        Ok(())
    }
    pub async fn get_sha1(&mut self) -> Result<String> {
        if let Some(s) = &self.sha1 {
            return Ok(s.clone());
        }
        if !self.resolved {
            self.resolve_remotes().await?;
        }
        if let Some(s) = &self.sha1 {
            Ok(s.clone())
        } else {
            Err(FetchError("Failed to fetch SHA1".to_string()).into())
        }
    }
    /**
     * Overwrites [self] with a reference to a global client
     */
    pub fn with_shared_client(mut self, client: reqwest::Client) -> Self {
        self.client = client.clone();
        return self;
    }
}
impl From<PackModDescription> for ModInfo {
    fn from(value: PackModDescription) -> Self {
        Self {
            config: WrappedModInfo::CurseMod(value),
            ..Default::default()
        }
    }
}
impl From<ModrinthFileInfo> for ModInfo {
    fn from(value: ModrinthFileInfo) -> Self {
        Self {
            config: WrappedModInfo::ModrinthMod(value),
            ..Default::default()
        }
    }
}

pub trait DepResolve {
    fn resolve_deps(self) -> Self;
}
impl DepResolve for Vec<ModInfo> {
    fn resolve_deps(self) -> Self {
        for m_info in self {}
        return Vec::default(); //FIXME
    }
}
