use std::path::Path;

use rust_search::SearchBuilder;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

// pub struct QuakeClientInfo {
//     pub name: String,
//     pub source_code_url: String,
// }

pub struct QuakeInstallation {
    pub pak0_path: String,
    pub id1_path: String,
    pub root_path: String,
}

impl QuakeInstallation {
    fn new_from_pak0_path(pak0_path: &str) -> QuakeInstallation {
        let parent_path = get_parent_path(pak0_path);
        let parent_name = Path::new(&parent_path).file_name().unwrap();
        let id1_path = if parent_name.eq("id1") && is_dir(&parent_path) { parent_path } else { "".to_string() };
        let root_path = if !id1_path.is_empty() { get_parent_path(&id1_path) } else { "".to_string() };

        return QuakeInstallation {
            pak0_path: pak0_path.to_string(),
            id1_path,
            root_path,
        };
    }
}

impl Serialize for QuakeInstallation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("QuakeInstallation", 3)?;
        state.serialize_field("pak0_path", &self.pak0_path)?;
        state.serialize_field("id1_path", &self.id1_path)?;
        state.serialize_field("root_path", &self.root_path)?;
        state.end()
    }
}


pub fn get_installations() -> Vec<QuakeInstallation> {
    let all_pak0_paths = find_pak0_paths("~");
    let mut result = Vec::new();

    for pak0_path in all_pak0_paths {
        result.push(QuakeInstallation::new_from_pak0_path(&pak0_path));
    }

    return result;
}


fn is_dir(path: &str) -> bool {
    Path::new(&path).is_dir()
}

fn get_parent_path(path: &str) -> String {
    let parent = Path::new(path).parent();

    return if parent == None {
        "".to_string()
    } else {
        parent.unwrap().display().to_string()
    };
}

fn find_pak0_paths(base_dir: &str) -> Vec<String> {
    let search: Vec<String> = SearchBuilder::default()
        .location(base_dir)
        .search_input("pak0.pak")
        .ignore_case()
        .strict()
        .limit(11)
        .build()
        .collect();

    let mut result: Vec<String> = vec![];

    for pak0_path in search {
        if Path::new(&pak0_path).is_file() {
            result.push(pak0_path)
        }
    }

    return result;
}
