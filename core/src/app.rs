use crate::modules::ModuleCollection;

pub struct App {
    pub modules: ModuleCollection,
}


pub fn create_app() -> App {
    let app = App {
        modules: ModuleCollection::new(),
    };
    return app;
}
