pub trait Module {
    fn info(&self) -> ModuleInfo;
    fn is_installed(&self) -> bool;
}

pub struct ModuleInfo {
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub website: String,
    pub repo: String,
}
