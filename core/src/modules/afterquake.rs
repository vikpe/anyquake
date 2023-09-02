use std::path::Path;
use std::string::ToString;

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::modules::ModuleLike;

const MODULE_ID: &str = "afterquake";

#[derive(Clone)]
pub struct AfterQuake;

#[async_trait]
impl ModuleLike for AfterQuake {
    fn id(&self) -> String {
        MODULE_ID.to_string()
    }

    fn is_installed(&self) -> bool {
        self.dir().has_file(Path::new("afterquake.zip"))
    }

    fn uninstall(&self) -> Result<String> {
        if !self.is_installed() {
            return Err(anyhow!("{} is not installed", self.id()));
        }

        self.dir().delete_file(Path::new("afterquake.zip"))?;
        Ok(format!("Successfully uninstalled {}", self.id()))
    }
}
