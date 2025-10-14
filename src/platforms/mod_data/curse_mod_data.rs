use crate::ModInfo;
use crate::platforms::curse::{APIFile, HashAlgo, PackModDescription, RelationType};
use color_eyre::Result;

impl From<PackModDescription> for ModInfo<PackModDescription, APIFile> {
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
impl ModInfo<PackModDescription, APIFile> {
    pub async fn resolve_remotes(&mut self) -> Result<()> {
        let resp = self
            .client
            .get(format!(
                "https://api.curseforge.com/v1/mods/{}/files/{}",
                self.config.project_id, self.config.file_id
            ))
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(super::FetchError(format!(
                "Could not resolve mod: http err {}",
                resp.status()
            ))
            .into());
        }
        let info = serde_json::from_str::<crate::platforms::curse::FileLookupResponse>(
            resp.text().await?.as_str(),
        )?;
        self.resolved_info = Some(info.data);
        if let Some(data) = &self.resolved_info {
            let hash = data
                .hashes
                .iter()
                .find(|h| h.algo == HashAlgo::Sha1)
                .take()
                .ok_or("No SHA1 hash in api response")
                .map_err(|e| super::FetchError(e.to_string()))
                .map(|f| f.value.clone())?;
            self.sha1 = Some(hash);
            self.deps = Some(
                data.dependencies
                    .iter()
                    .filter(|dep| dep.relation_type == RelationType::RequiredDependency)
                    .map(|dep| super::DependencyInfo {
                        curse_project_id: Some(dep.mod_id),
                        sha1: None,
                    })
                    .collect(),
            );
        } else {
            return Err(super::FetchError("Bad response".to_string()).into());
        }
        Ok(())
    }
    /**
     * Overwrites [self] with a reference to a global client
     */
    pub fn with_shared_client(mut self, client: reqwest::Client) -> Self {
        self.client = client.clone();
        return self;
    }
}
