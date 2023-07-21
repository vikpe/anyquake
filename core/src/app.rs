use crate::module::Module;
use crate::modules;

pub struct App {
    pub modules: Vec<Box<dyn Module>>,
}

pub fn create_app() -> App {
    let ezquake = Box::new(modules::ezquake::EzQuake {});
    let afterquake = Box::new(modules::afterquake::AfterQuake {});
    let app = App {
        modules: vec![ezquake, afterquake]
    };
    return app;
}
