use crate::ModInfo;
use crate::platforms::curse::{
    APIFile, HashAlgo, PackModDescription, RelationType, SortableGameVersion,
};
use color_eyre::{Result, eyre};

impl From<PackModDescription> for ModInfo<PackModDescription, APIFile, CurseDependency> {
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
impl ModInfo<PackModDescription, APIFile, CurseDependency> {
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
                    .map(|dep| CurseDependency(dep.mod_id))
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
    pub async fn resolve(&mut self) -> Result<()> {
        let d_arr = match &self.deps {
            None => {
                tracing::event!(tracing::Level::DEBUG, "No dependencies to fetch");
                &vec![]
            }
            Some(d) => d,
        };
        let mut dep_hashes: Vec<String> = vec![];
        let (svn, curse_ver_id) =
            (|ver: &SortableGameVersion| -> (String, u32) {
                (
                    ver.game_version.clone(),
                    ver.game_version_type_id.unwrap().clone(),
                )
            })(&(self.resolved_info.as_ref().unwrap().sortable_game_versions[0]));
        for dep in d_arr {}

        Ok(())
    }
}
#[derive(Debug)]
pub struct CurseDependency(u32);
