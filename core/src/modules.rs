use std::vec::IntoIter;

use anyhow::Result;
use async_trait::async_trait;

use crate::assets::RestrictedDir;
// use crate::modules::afterquake::AfterQuake;
use crate::modules::ezquake::EzQuake;

pub mod ezquake;

#[async_trait]
pub trait ModuleLike {
    fn dir(&self) -> &RestrictedDir;
    fn info(&self) -> ModuleInfo;
    fn is_installed(&self) -> bool;
    async fn install(&self) -> Result<()>;
    fn uninstall(&self) -> Result<()>;
}

pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub website: String,
    pub repo: String,
}

pub struct ModuleCollection {
    pub ezquake: EzQuake,
}


impl ModuleCollection {
    pub fn new() -> Self {
        ModuleCollection {
            // afterquake: AfterQuake,
            ezquake: EzQuake::new(),
        }
    }

    pub fn all(&self) -> Vec<Box<dyn ModuleLike>> {
        vec![
            // Box::new(AfterQuake),
            Box::new(EzQuake::new()),
        ]
    }

    pub fn into_iter(&self) -> IntoIter<Box<dyn ModuleLike>> {
        self.all().into_iter()
    }

    pub fn names(&self) -> Vec<String> {
        self.into_iter().map(|m| m.info().name).collect()
    }

    pub fn by_id(&self, id: String) -> Option<Box<dyn ModuleLike>> {
        self.into_iter().find(|m| m.info().id == id)
    }
}
