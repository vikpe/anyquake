use std::path::Path;
use std::vec::IntoIter;

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::anyquake;
use crate::modules::afterquake::AfterQuake;
use crate::modules::ezquake::EzQuake;
use crate::repo;
use crate::repo::{Module, Release, ReleaseAsset};
use crate::restricted_dir::RestrictedDir;

pub mod afterquake;
pub mod ezquake;

pub fn get_module_dir(module_id: &str) -> RestrictedDir {
    RestrictedDir::new(&anyquake::get_root_dir_path().join(module_id))
}

#[async_trait]
pub trait ModuleLike {
    fn id(&self) -> String;
    fn dir(&self) -> RestrictedDir {
        get_module_dir(&self.id())
    }
    async fn info(&self) -> Result<Module> {
        repo::get_module_info(&self.id()).await
    }
    async fn versions(&self) -> Result<Vec<String>> {
        repo::get_module_info(&self.id())
            .await?
            .releases
            .iter()
            .map(|r| Ok(r.version.clone()))
            .collect()
    }
    fn is_installed(&self) -> bool;
    async fn install(&self) -> Result<String> {
        if self.is_installed() {
            return Err(anyhow!("{} is already installed", self.id()));
        }

        let info = self.info().await?;

        match info.releases.len() {
            0 => Err(anyhow!("No releases found")),
            _ => {
                let latest_release: &Release =
                    info.releases.first().ok_or(anyhow!("No releases found"))?;
                let first_asset: &ReleaseAsset = latest_release
                    .assets
                    .first()
                    .ok_or(anyhow!("No assets found"))?;
                self.dir().download(&first_asset.url, Path::new("")).await?;
                return Ok(format!("Successfully installed {}", self.id()));
            }
        }
    }
    fn uninstall(&self) -> Result<String>;
}

pub struct ModuleCollection;

impl ModuleCollection {
    pub fn all(&self) -> Vec<Box<dyn ModuleLike + Sync + Send>> {
        vec![Box::new(AfterQuake {}), Box::new(EzQuake {})]
    }

    pub fn into_iter(&self) -> IntoIter<Box<dyn ModuleLike + Sync + Send>> {
        self.all().into_iter()
    }

    pub fn ids(&self) -> Vec<String> {
        self.into_iter().map(|m| m.id()).collect()
    }

    pub fn by_id(&self, id: &str) -> Result<Box<dyn ModuleLike + Sync + Send>> {
        return match self.into_iter().find(|m| m.id() == id) {
            Some(module) => Ok(module),
            None => Err(anyhow!(
                "Module is {} not supported. Supported modules: {}",
                id,
                self.ids().join(", ")
            )),
        };
    }
}
