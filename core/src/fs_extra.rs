use std::path::PathBuf;
use std::fs;

pub fn create_dir_all(path: &PathBuf) -> Result<(), String> {
    if let Err(_) = fs::metadata(path) {
        if let Err(err) = fs::create_dir_all(path) {
            return Err(err.to_string());
        }
    }
    Ok(())
}

pub fn write_file(dir_path: PathBuf, filename: &str, content: &str) -> Result<(), String> {
    create_dir_all(&dir_path)?;
    let file_path = dir_path.join(filename);
    if let Err(err) = fs::write(file_path, content) {
        return Err(err.to_string());
    }
    Ok(())
}
