extern crate dirs;

use std::path::PathBuf;

pub fn get_root_dir_path() -> PathBuf {
    dirs::home_dir()
        .expect("should get home dir")
        .join("anyquake")
}
