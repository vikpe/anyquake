use std::path::PathBuf;

use rust_search::SearchBuilder;

pub fn find_pak0_paths(path: &PathBuf) -> Vec<String> {
    let result: Vec<String> = SearchBuilder::default()
        .location(path.display().to_string())
        .search_input("pak0.pak")
        .ignore_case()
        .strict()
        .limit(11)
        .build()
        .filter(|p| PathBuf::from(p).is_file())
        .collect();

    return result;
}