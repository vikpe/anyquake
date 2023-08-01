extern crate dirs;

use std::path::PathBuf;
use std::string::ToString;

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::modules::ModuleLike;
use crate::repo::{Release, ReleaseAsset};

const MODULE_ID: &str = "ezquake";

#[derive(Clone)]
pub struct EzQuake;

#[async_trait]
impl ModuleLike for EzQuake {
    fn id(&self) -> String {
        return MODULE_ID.to_string();
    }

    fn is_installed(&self) -> bool {
        self.dir().has_file(&PathBuf::from("ezquake_releases.json"))
    }

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

    fn uninstall(&self) -> Result<()> {
        if !self.is_installed() {
            return Err(anyhow!("{} is not installed", self.id()));
        }

        self.dir()
            .delete_file(&PathBuf::from("ezquake_releases.json"))
    }
}
