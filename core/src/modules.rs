pub mod afterquake;
pub mod ezquake;

pub trait ModuleLike {
    fn info(&self) -> ModuleInfo;
    fn is_installed(&self) -> bool;
}

pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub website: String,
    pub repo: String,
}
