use std::path::PathBuf;
use std::vec::IntoIter;

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::assets::RestrictedDir;
use crate::modules::afterquake::AfterQuake;
use crate::modules::ezquake::EzQuake;
use crate::repo;
use crate::repo::{Module, Release, ReleaseAsset};

pub mod afterquake;
pub mod ezquake;

pub fn get_module_dir(id: &str) -> RestrictedDir {
    let anyquake_path = PathBuf::from("/home/vikpe/anyquake");
    RestrictedDir::new(&anyquake_path.join(id))
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
    async fn install(&self) -> Result<()> {
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
                self.dir()
                    .download(&first_asset.url, &PathBuf::from(""))
                    .await?;
                return Ok(());
            }
        }
    }
    fn uninstall(&self) -> Result<()>;
}

pub struct ModuleCollection;

impl ModuleCollection {
    pub fn all(&self) -> Vec<Box<dyn ModuleLike + Sync>> {
        vec![Box::new(AfterQuake {}), Box::new(EzQuake {})]
    }

    pub fn into_iter(&self) -> IntoIter<Box<dyn ModuleLike + Sync>> {
        self.all().into_iter()
    }

    pub fn ids(&self) -> Vec<String> {
        self.into_iter().map(|m| m.id()).collect()
    }

    pub fn by_id(&self, id: &str) -> Option<Box<dyn ModuleLike + Sync>> {
        self.into_iter().find(|m| m.id() == id)
    }
}
