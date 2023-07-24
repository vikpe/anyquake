use crate::modules::{afterquake, ezquake, ModuleLike};

pub struct DefaultModules {
    pub afterquake: afterquake::AfterQuake,
    pub ezquake: ezquake::EzQuake,
}

impl DefaultModules {
    pub fn new() -> DefaultModules {
        DefaultModules {
            afterquake: afterquake::AfterQuake {},
            ezquake: ezquake::EzQuake {},
        }
    }

    pub fn all(&self) -> Vec<Box<dyn ModuleLike>> {
        vec![
            Box::new(self.afterquake.clone()),
            Box::new(self.ezquake.clone()),
        ]
    }

    pub fn names(&self) -> Vec<String> {
        self.all().into_iter().map(|m| m.info().name).collect()
    }

    pub fn by_id(&self, id: String) -> Option<Box<dyn ModuleLike>> {
        self.all().into_iter().find(|m| m.info().id == id)
    }
}
