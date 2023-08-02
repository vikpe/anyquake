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

impl std::string::ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::List {} => "list".into(),
            Command::Info { module_id: id } => format!("info {}", id),
            Command::Versions { module_id: id } => format!("versions {}", id),
            Command::Install { module_id: id } => format!("install {}", id),
            Command::Uninstall { module_id: id } => format!("uninstall {}", id),
        }
    }
}
