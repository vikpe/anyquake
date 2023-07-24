use crate::modules::{ModuleInfo, ModuleLike};

#[derive(Clone)]
pub struct AfterQuake;

impl ModuleLike for AfterQuake {
    fn is_installed(&self) -> bool {
        true
    }

    fn info(&self) -> ModuleInfo {
        return ModuleInfo {
            id: String::from("afterquake"),
            name: String::from("AfterQuake"),
            description: String::from("todo"),
            website: String::from("todo"),
            repo: String::from("todo"),
        };
    }
}
