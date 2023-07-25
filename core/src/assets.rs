use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::fs_extra;

pub struct BaseDir {
    root: PathBuf,
}

impl BaseDir {
    pub fn new(dir_name: &str) -> Result<Self> {
        let home_path = match dirs::home_dir() {
            Some(path) => path,
            None => return Err(anyhow!("Unable to locate home dir"))
        };

        Ok(BaseDir { root: home_path.join(dir_name) })
    }

    pub fn dir_path(&self, path: &PathBuf) -> PathBuf {
        self.root.join(path)
    }

    pub fn file_path(&self, dir_path: &PathBuf, filename: &str) -> PathBuf {
        self.root.join(dir_path).join(filename)
    }

    pub fn write_file(&self, dir_path: &PathBuf, filename: &str, content: &str) -> Result<()> {
        fs_extra::write_file(self.dir_path(dir_path), filename, content)
    }
}

pub struct Assets {
    pub dir: BaseDir,
}

impl Assets {
    pub fn new() -> Result<Self> {
        let dir = match BaseDir::new("anyquake") {
            Ok(dir) => dir,
            Err(e) => return Err(anyhow!(e.to_string()))
        };
        Ok(Assets { dir })
    }
}
