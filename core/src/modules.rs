use std::path::PathBuf;
use std::vec::IntoIter;

use anyhow::Result;
use async_trait::async_trait;

use crate::assets::RestrictedDir;
use crate::modules::ezquake::EzQuake;
use crate::repo;
use crate::repo::Module;

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
    async fn install(&self) -> Result<()>;
    fn uninstall(&self) -> Result<()>;
}

pub struct ModuleCollection {
    pub ezquake: EzQuake,
}

impl ModuleCollection {
    pub fn new() -> Self {
        ModuleCollection {
            ezquake: EzQuake {},
        }
    }

    pub fn all(&self) -> Vec<Box<dyn ModuleLike + Sync>> {
        vec![Box::new(EzQuake {})]
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
