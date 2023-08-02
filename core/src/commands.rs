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
