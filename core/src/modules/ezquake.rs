extern crate dirs;

use std::fs::metadata;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::assets::RestrictedDir;
use crate::modules::{ModuleInfo, ModuleLike};

#[derive(Clone)]
pub struct EzQuake {
    id: String,
    name: String,
    description: String,
    website: String,
    repo: String,
    dir: RestrictedDir,
}

impl EzQuake {
    pub fn new() -> Self {
        let anyquake_path = PathBuf::from("/home/vikpe/anyquake");

        EzQuake {
            id: String::from("ezquake"),
            name: String::from("EzQuake"),
            description: String::from("todo"),
            website: String::from("https://ezquake.com/"),
            repo: String::from("https://github.com/QW-Group/ezquake-source"),
            dir: RestrictedDir::new(&anyquake_path.join("ezquake")),
        }
    }
}

#[async_trait]
impl ModuleLike for EzQuake {
    fn dir(&self) -> &RestrictedDir {
        &self.dir
    }

    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            website: self.website.clone(),
            repo: self.repo.clone(),
        }
    }

    fn is_installed(&self) -> bool {
        let file_path = self.dir.path.join("ezquake_releases.json");
        println!("{:?}", file_path);
        metadata(file_path).is_ok()
    }

    async fn install(&self) -> Result<()> {
        if self.is_installed() {
            return Err(anyhow!("{} is already installed", self.info().name));
        }

        let url = String::from("https://raw.githubusercontent.com/vikpe/qw-data/main/github/ezquake_releases.json");

        self.dir.download(&url, &PathBuf::from("ezquake_releases.json")).await
    }

    fn uninstall(&self) -> Result<()> {
        if !self.is_installed() {
            return Err(anyhow!("{} is not installed", self.info().name));
        }

        self.dir.delete_file(&PathBuf::from("ezquake_releases.json"))
    }
}
