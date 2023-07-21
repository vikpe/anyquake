use crate::module::Module;
use crate::module::afterquake::AfterQuake;
use crate::module::ezquake::EzQuake;

pub struct App {
    pub modules: Vec<Box<dyn Module>>,
}


pub fn create_app() -> App {
    let ezquake = Box::new(EzQuake {});
    let afterquake = Box::new(AfterQuake {});
    let app = App {
        modules: vec![ezquake, afterquake]
    };
    return app;
}
