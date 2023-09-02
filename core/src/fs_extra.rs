use std::fs;
use std::path::Path;

use anyhow::Result;

pub fn create_dir_all(path: &Path) -> Result<()> {
    if fs::metadata(path).is_err() {
        fs::create_dir_all(path)?
    }
    Ok(())
}

pub fn write_file(dir_path: &Path, filename: &str, content: &str) -> Result<()> {
    create_dir_all(&dir_path)?;
    let file_path = dir_path.join(filename);
    fs::write(file_path, content)?;
    Ok(())
}
