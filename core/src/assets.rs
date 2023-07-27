use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::download::download;
use crate::fs_extra;

#[derive(Clone)]
pub struct RestrictedDir {
    pub path: PathBuf,
}

impl RestrictedDir {
    pub fn new(path: &PathBuf) -> Self {
        Self { path: path.clone() }
    }

    pub fn validate_path(&self, path: &PathBuf) -> Result<()> {
        match path.starts_with(&self.path) {
            true => Ok(()),
            false => Err(anyhow!("Path '{}' is not within the restricted dir", path.display()))
        }
    }

    pub fn get_path_to(&self, path: &PathBuf) -> PathBuf {
        self.path.join(path)
    }

    pub fn has_file(&self, path: &PathBuf) -> bool {
        self.get_path_to(path).is_file()
    }

    pub fn delete_file(&self, file_path: &PathBuf) -> Result<()> {
        if !self.has_file(file_path) {
            return Ok(());
        }

        let path_rel = self.get_path_to(file_path);
        self.validate_path(&path_rel)?;

        fs::remove_file(path_rel)?;
        Ok(())
    }

    pub fn create_dir(&self, path: &PathBuf) -> Result<()> {
        let path_rel = self.get_path_to(path);
        self.validate_path(&path_rel)?;

        fs_extra::create_dir_all(&path_rel)
    }

    pub async fn download(&self, url: &str, dest: &PathBuf) -> Result<()> {
        let dest_rel = self.get_path_to(dest);
        self.validate_path(&dest_rel)?;

        download(&url, &dest_rel).await
    }
}
