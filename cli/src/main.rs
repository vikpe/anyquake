extern crate prettytable;

use anyhow::Result;
use clap::{Parser, Subcommand};
use prettytable::{Cell, Row, Table};
use tokio;

use anyquake_core::modules::ModuleCollection;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List {},
    Install {
        module: String,
    },
    Uninstall {
        module: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let modules: ModuleCollection = ModuleCollection::new();

    match &cli.command {
        Some(Commands::Install { module: name }) => {
            if let Some(module) = modules.by_id(String::from(name)) {
                match module.install().await {
                    Ok(_) => println!("Installed {}[{}]!", name, module.info().id),
                    Err(e) => println!("Failed to install {}[{}]! {}", name, module.info().id, e),
                }
            } else {
                println!("Module is {} not supported", name);
                println!("Supported modules: {}", modules.names().join(", "));
            }
        }
        Some(Commands::Uninstall { module: name }) => {
            if let Some(module) = modules.by_id(String::from(name)) {
                match module.uninstall() {
                    Ok(_) => println!("Uninstalled {}[{}]!", name, module.info().id),
                    Err(e) => println!("Failed to uninstall {}[{}]! {}", name, module.info().id, e),
                }
            } else {
                println!("Module is {} not installed", name);
                let installed_modules: Vec<String> = modules.into_iter().filter(|m| m.is_installed()).map(|m| m.info().name).collect();

                println!("Installed modules: {}", installed_modules.join(", "));
            }
        }
        Some(Commands::List {}) => {
            let mut table = Table::new();

            table.add_row(Row::new(vec![
                Cell::new("name"),
                Cell::new("description"),
            ]));

            modules.into_iter()
                .filter(|m| m.is_installed())
                .for_each(|m| {
                    let info = &m.info();

                    table.add_row(Row::new(vec![
                        Cell::new(&info.name),
                        Cell::new(&info.description),
                    ]));
                });

            table.printstd();
        }
        None => {}
    }

    Ok(())
}

