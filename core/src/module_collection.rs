use crate::modules::{afterquake, ezquake, ModuleLike};

pub struct DefaultModules {
    pub afterquake: afterquake::AfterQuake,
    pub ezquake: ezquake::EzQuake,
}

impl DefaultModules {
    pub fn new() -> DefaultModules {
        return DefaultModules {
            afterquake: afterquake::AfterQuake {},
            ezquake: ezquake::EzQuake {},
        };
    }

    pub fn all(&self) -> Vec<Box<dyn ModuleLike>> {
        return vec![
            Box::new(self.afterquake.clone()),
            Box::new(self.ezquake.clone()),
        ];
    }

    pub fn names(&self) -> Vec<String> {
        self.all().into_iter().map(|e| e.info().name).collect()
    }
}
