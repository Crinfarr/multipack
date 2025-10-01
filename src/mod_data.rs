use std::{default, pin::Pin, ptr::null, sync::Arc};

use crate::platforms::{curse::CurseFileDescription, mr::ModrinthJarDescription};

#[derive(Default)]
enum WrappedModInfo {
    CurseMod(crate::platforms::curse::CurseFileDescription),
    ModrinthMod(crate::platforms::mr::ModrinthJarDescription),
    #[default]
    Undefined
}
enum LookupClient {
    Ref(Arc<reqwest::Client>),
    Internal(reqwest::Client)
}
impl Default for LookupClient {
    fn default() -> Self {
        Self::Internal(reqwest::Client::default())
    }
}

#[derive(Debug, Clone)]
struct FetchError<'a> {
    message:&'static str,
    wrapped:Option<&'a Box<dyn std::error::Error>>,
}
#[derive(Default)]
pub struct ModInfo<'a> {
    config:WrappedModInfo,
    sha1:Option<&'a str>,
    client:LookupClient
}
impl ModInfo<'_> {
    pub async fn get_sha1(&mut self) -> Result<&str, FetchError> {
        if let Some(s) = self.sha1 {
            return Ok(&s);
        }
        match &self.config {
            WrappedModInfo::CurseMod(desc) => {
            }
            WrappedModInfo::ModrinthMod(desc) => {
            }
            WrappedModInfo::Undefined => self.sha1 = None,
        }
        return self.sha1.map_or(Err(FetchError {message: "Failed to resolve sha1", wrapped: None}), |s| Ok(s.as_str()));
    }
    /**
     * Overwrites [self] with a reference to a global client
     */
    pub fn with_shared_client(mut self, client:Arc<reqwest::Client>) -> Self {
        self.client = LookupClient::Ref(client.clone());
        return self;
    }
}
impl From<CurseFileDescription> for ModInfo<'_> {
    fn from(value: CurseFileDescription) -> Self {
        Self {
            config: WrappedModInfo::CurseMod(value),
            ..Default::default()
        }
    }
}
impl From<ModrinthJarDescription> for ModInfo<'_> {
    fn from(value: ModrinthJarDescription) -> Self {
        Self {
            config: WrappedModInfo::ModrinthMod(value),
            ..Default::default()
        }
    }
}

trait DepResolve {
    fn resolve_deps(self) -> Self;
}