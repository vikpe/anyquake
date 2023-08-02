extern crate dirs;

use std::path::PathBuf;
use std::string::ToString;

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::modules::ModuleLike;

const MODULE_ID: &str = "ezquake";

#[derive(Clone)]
pub struct EzQuake;

#[async_trait]
impl ModuleLike for EzQuake {
    fn id(&self) -> String {
        return MODULE_ID.to_string();
    }

    fn is_installed(&self) -> bool {
        self.dir().has_file(&PathBuf::from("ezquake.exe"))
    }

    fn uninstall(&self) -> Result<String> {
        if !self.is_installed() {
            return Err(anyhow!("{} is not installed", self.id()));
        }

        self.dir().delete_file(&PathBuf::from("ezquake.exe"))?;
        Ok(format!("Successfully uninstalled {}", self.id()))
    }
}
