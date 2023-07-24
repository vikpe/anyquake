extern crate dirs;

use std::path::PathBuf;

use crate::afs::aq_write_file;
use crate::modules::{ModuleInfo, ModuleLike};

#[derive(Clone)]
pub struct EzQuake;

impl ModuleLike for EzQuake {
    fn is_installed(&self) -> bool {
        false
    }

    fn info(&self) -> ModuleInfo {
        return ModuleInfo {
            id: String::from("ezquake"),
            name: String::from("EzQuake"),
            description: String::from("todo"),
            website: String::from("https://ezquake.com/"),
            repo: String::from("https://github.com/QW-Group/ezquake-source"),
        };
    }

    fn install(&self) -> Result<(), String> {
        aq_write_file(&PathBuf::from("eh"), "ezquake2.txt", "hello")
    }
}
