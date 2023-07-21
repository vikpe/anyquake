use crate::module::{Module, ModuleInfo};

pub struct AfterQuake {}

impl Module for AfterQuake {
    fn is_installed(&self) -> bool {
        true
    }

    fn info(&self) -> ModuleInfo {
        return ModuleInfo {
            identifier: String::from("afterquake"),
            name: String::from("AfterQuake"),
            description: String::from("todo"),
            website: String::from("todo"),
            repo: String::from("todo"),
        };
    }
}
