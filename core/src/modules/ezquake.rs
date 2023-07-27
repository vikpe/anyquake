extern crate dirs;

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures_util::future::join_all;

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
    asset_list: Vec<String>,
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
            asset_list: vec![
                String::from("https://raw.githubusercontent.com/vikpe/qw-data/main/github/ezquake_releases.json"),
                String::from("https://raw.githubusercontent.com/vikpe/qw-data/main/github/ktx_latest_release.json"),
            ],
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
        self.dir.has_file(&PathBuf::from("ezquake_releases.json"))
    }

    async fn install(&self) -> Result<()> {
        if self.is_installed() {
            return Err(anyhow!("{} is already installed", self.info().name));
        }

        let dest = PathBuf::from(".");
        let tasks = self.asset_list.iter().map(|url| self.dir.download(&url, &dest));
        let result = join_all(tasks).await;

        for result in result {
            if let Err(e) = result {
                eprintln!("Error while downloading file: {:?}", e);
            }
        }

        Ok(())
    }

    fn uninstall(&self) -> Result<()> {
        if !self.is_installed() {
            return Err(anyhow!("{} is not installed", self.info().name));
        }

        self.dir.delete_file(&PathBuf::from("ezquake_releases.json"))
    }
}
