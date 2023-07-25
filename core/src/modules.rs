use std::vec::IntoIter;

use anyhow::Result;

// use crate::modules::afterquake::AfterQuake;
use crate::modules::ezquake::EzQuake;

//pub mod afterquake;
pub mod ezquake;

pub trait ModuleLike {
    fn info(&self) -> ModuleInfo;
    fn is_installed(&self) -> bool;
    fn install(&self) -> Result<()>;
}

pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub website: String,
    pub repo: String,
}

pub struct ModuleCollection {
    // pub afterquake: AfterQuake,
    pub ezquake: EzQuake,
}

impl Default for ModuleCollection {
    fn default() -> Self {
        ModuleCollection {
            // afterquake: AfterQuake,
            ezquake: EzQuake,
        }
    }
}

impl ModuleCollection {
    pub fn new() -> Self {
        ModuleCollection {
            ..Default::default()
        }
    }

    pub fn all(&self) -> Vec<Box<dyn ModuleLike>> {
        vec![
            // Box::new(AfterQuake),
            Box::new(EzQuake),
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
