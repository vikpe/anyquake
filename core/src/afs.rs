use std::fs;
use std::path::PathBuf;

// Anyquake --------------------------------------------------------

pub fn aq_dir_path() -> Result<PathBuf, String> {
    match dirs::home_dir() {
        Some(home_path) => Ok(home_path.join("anyquake")),
        None => Err("Unable to locate home dir".to_string()),
    }
}

pub fn aq_ensure_dir_exist(path: &PathBuf) -> Result<(), String> {
    let anyquake_path = aq_dir_path()?;
    std_create_dir(&anyquake_path.join(path))
}

pub fn aq_write_file(dir_path: &PathBuf, filename: &str, content: &str) -> Result<(), String> {
    let anyquake_path = aq_dir_path()?;
    std_write_file(
        anyquake_path.join(dir_path),
        filename,
        content,
    )
}


// STD --------------------------------------------------------

fn std_create_dir(path: &PathBuf) -> Result<(), String> {
    if let Err(_) = fs::metadata(path) {
        if let Err(err) = fs::create_dir_all(path) {
            return Err(err.to_string());
        }
    }
    Ok(())
}

fn std_write_file(dir_path: PathBuf, filename: &str, content: &str) -> Result<(), String> {
    std_create_dir(&dir_path)?;
    let file_path = dir_path.join(filename);
    if let Err(err) = fs::write(file_path, content) {
        return Err(err.to_string());
    }
    Ok(())
}


