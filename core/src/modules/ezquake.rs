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
}
