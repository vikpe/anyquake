use crate::modules::{afterquake, ezquake, Module};

pub struct App {
    pub modules: Vec<Box<dyn Module>>,
}

pub fn create_app() -> App {
    let ezquake = Box::new(ezquake::EzQuake {});
    let afterquake = Box::new(afterquake::AfterQuake {});
    let app = App {
        modules: vec![ezquake, afterquake]
    };
    return app;
}
