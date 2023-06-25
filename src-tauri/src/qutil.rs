use std::ops::Index;
use std::path::Path;

use rust_search::{SearchBuilder};

pub struct QuakeInstallation {
    pub pak0_path: String,
    pub root_dir_path: String,
}

pub fn get_installation() -> QuakeInstallation {
    let pak0_path = find_pak0_path("~");
    let pak0_path_clone = &pak0_path.clone();
    return QuakeInstallation {
        pak0_path,
        root_dir_path: get_parent_dir(pak0_path_clone),
    };
}


fn get_parent_dir(path: &str) -> String {
    let parent = Path::new(path).parent();

    return if parent == None {
        "".to_string()
    } else {
        parent.unwrap().display().to_string()
    };
}

fn find_pak0_path(base_dir: &str) -> String {
    let search: Vec<String> = SearchBuilder::default()
        .location(base_dir)
        .search_input("pak0.pak")
        .ignore_case()
        .strict()
        .limit(1)
        .build()
        .collect();

    return if search.is_empty() {
        "".to_string()
    } else {
        search.index(0).to_string()
    };
}
