extern crate prettytable;

use anyhow::Result;
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

pub fn get_module_by_id(id: &str) -> Option<Box<dyn ModuleLike + Sync>> {
    let modules: ModuleCollection = ModuleCollection::new();

    return match modules.by_id(id) {
        Some(module) => Some(module),
        None => {
            println!("Module is {} not supported", id);
            println!("Supported modules: {}", modules.ids().join(", "));
            return None;
        }
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let modules: ModuleCollection = ModuleCollection::new();

    match &cli.command {
        Some(Commands::Info { module_id: id }) => {
            if let Some(module) = get_module_by_id(id) {
                let info = module.info().await?;
                println!("info: {:?}", info);
            }
        }
        Some(Commands::Versions { module_id: id }) => {
            if let Some(module) = get_module_by_id(id) {
                let versions = module.versions().await?;
                println!("versions: {:?}", versions);
            }
        }
        Some(Commands::Install { module_id: id }) => {
            if let Some(module) = get_module_by_id(id) {
                if let Err(e) = module.install().await {
                    println!("{e}")
                }
            }
        }
        Some(Commands::Uninstall { module_id: id }) => {
            if let Some(module) = get_module_by_id(id) {
                if let Err(e) = module.uninstall() {
                    println!("{e}")
                }
            }
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

            table.printstd();
        }
        None => {}
    }

    Ok(())
}
