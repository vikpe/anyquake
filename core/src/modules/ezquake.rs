extern crate dirs;

use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::assets::Assets;
use crate::modules::{ModuleInfo, ModuleLike};

#[derive(Clone)]
pub struct EzQuake;

impl ModuleLike for EzQuake {
    fn is_installed(&self) -> bool {
        false
    }

    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            id: String::from("ezquake"),
            name: String::from("EzQuake"),
            description: String::from("todo"),
            website: String::from("https://ezquake.com/"),
            repo: String::from("https://github.com/QW-Group/ezquake-source"),
        }
    }

    fn install(&self) -> Result<()> {
        let assets = match Assets::new() {
            Ok(a) => a,
            Err(e) => return Err(anyhow!(e.to_string())),
        };
        assets.dir.write_file(&PathBuf::from("eh"), "ezquake.txt", "hello")
    }
}
