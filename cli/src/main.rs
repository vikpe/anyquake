extern crate prettytable;

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use prettytable::{Cell, Row, Table};

use anyquake_core::commands::Command;
use anyquake_core::modules::ModuleCollection;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let modules: ModuleCollection = ModuleCollection {};
    let result: Result<String, Error> = match &cli.command {
        Some(Command::Info { module_id: id }) => {
            let info = modules.by_id(id)?.info().await?;
            Ok(format!("{} info: {:?}", id, info))
        }
        Some(Command::Versions { module_id: id }) => {
            let versions = modules.by_id(id)?.versions().await?;
            Ok(format!("{} versions: {:?}", id, versions))
        }
        Some(Command::Install { module_id: id }) => {
            modules.by_id(id)?.install().await
        }
        Some(Command::Uninstall { module_id: id }) => {
            modules.by_id(id)?.uninstall()
        }
        Some(Command::List {}) => {
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
        None => Err(anyhow!("Invalid command.")),
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => println!("{}", error),
    }

    Ok(())
}
