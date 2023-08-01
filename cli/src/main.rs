extern crate prettytable;

use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use prettytable::{Cell, Row, Table};

use anyquake_core::modules::{ModuleCollection, ModuleLike};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List {},
    Info { module_id: String },
    Versions { module_id: String },
    Install { module_id: String },
    Uninstall { module_id: String },
}

pub fn get_module_by_id(id: &str) -> Result<Box<dyn ModuleLike + Sync>> {
    let modules: ModuleCollection = ModuleCollection {};

    return match modules.by_id(id) {
        Some(module) => Ok(module),
        None => Err(anyhow::anyhow!(
            "Module is {} not supported. Supported modules: {}",
            id,
            modules.ids().join(", ")
        )),
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let modules: ModuleCollection = ModuleCollection {};
    let result: Result<String, Error> = match &cli.command {
        Some(Commands::Info { module_id: id }) => {
            let info = get_module_by_id(id)?.info().await?;
            Ok(format!("{} info: {:?}", id, info))
        }
        Some(Commands::Versions { module_id: id }) => {
            let versions = get_module_by_id(id)?.versions().await?;
            Ok(format!("{} versions: {:?}", id, versions))
        }
        Some(Commands::Install { module_id: id }) => {
            get_module_by_id(id)?.install().await?;
            Ok(format!("successfully installed {}", id))
        }
        Some(Commands::Uninstall { module_id: id }) => {
            get_module_by_id(id)?.uninstall()?;
            Ok(format!("successfully uninstalled {}", id))
        }
        Some(Commands::List {}) => {
            let mut table = Table::new();

            table.add_row(Row::new(vec![Cell::new("id")]));

            modules
                .into_iter()
                .filter(|m| m.is_installed())
                .for_each(|m| {
                    table.add_row(Row::new(vec![Cell::new(&m.id())]));
                });

            Ok(table.to_string())
        }
        None => Err(anyhow::anyhow!("Invalid command.")),
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => println!("{}", error),
    }

    Ok(())
}
