use std::fmt::Formatter;

use clap;
use serde;

#[derive(clap::Subcommand, serde::Deserialize, serde::Serialize, Debug)]
pub enum Command {
    List {},
    Info { module_id: String },
    Versions { module_id: String },
    Install { module_id: String },
    Uninstall { module_id: String },
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::List {} => write!(f, "list"),
            Command::Info { module_id: id } => write!(f, "info {}", id),
            Command::Versions { module_id: id } => write!(f, "versions {}", id),
            Command::Install { module_id: id } => write!(f, "install {}", id),
            Command::Uninstall { module_id: id } => write!(f, "uninstall {}", id),
        }
    }
}
