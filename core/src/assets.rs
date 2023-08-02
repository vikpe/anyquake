use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

use crate::download::download;
use crate::fs_extra;

#[derive(Clone)]
pub struct RestrictedDir {
    pub path: PathBuf,
}

impl RestrictedDir {
    pub fn new(path: &Path) -> Self {
        Self { path: path.to_path_buf() }
    }

    pub fn validate_path(&self, path: &Path) -> Result<()> {
        match path.starts_with(&self.path) {
            true => Ok(()),
            false => Err(anyhow!(
                "Path '{}' is not within the restricted dir",
                path.display()
            )),
        }
    }

    pub fn get_abs_path_to(&self, path: &PathBuf) -> PathBuf {
        self.path.join(path)
    }

    pub fn has_file(&self, path: &PathBuf) -> bool {
        self.get_abs_path_to(path).is_file()
    }

    pub fn delete_file(&self, file_path: &PathBuf) -> Result<()> {
        if !self.has_file(file_path) {
            return Ok(());
        }

        let path_abs = self.get_abs_path_to(file_path);
        self.validate_path(&path_abs)?;

        fs::remove_file(path_abs)?;
        Ok(())
    }

    pub fn create_dir(&self, path: &PathBuf) -> Result<()> {
        let path_abs = self.get_abs_path_to(path);
        self.validate_path(&path_abs)?;

        fs_extra::create_dir_all(&path_abs)
    }

    pub async fn download(&self, url: &str, dest: &PathBuf) -> Result<()> {
        let path_abs = self.get_abs_path_to(dest);
        self.validate_path(&path_abs)?;

        println!("Creating dir '{}'", path_abs.display());
        self.create_dir(dest)?;
        println!("Downloading '{}' to '{}'", url, path_abs.display());

        download(url, &path_abs).await
    }
}
