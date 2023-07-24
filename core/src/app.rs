use crate::module_collection::DefaultModules;

pub struct App {
    pub modules: DefaultModules,
}


pub fn create_app() -> App {
    let app = App {
        modules: DefaultModules::new()
    };
    return app;
}
