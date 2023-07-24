use clap::{Parser, Subcommand};

use anyquake_core::app::{App, create_app};

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

fn main() {
    let cli = Cli::parse();
    let app: App = create_app();

    match &cli.command {
        Some(Commands::Install { module: name }) => {
            // if not installed
            println!("Install: {}", name);

            // else
            // print info
        }
        Some(Commands::Uninstall { module: name }) => {
            // if installed
            println!("Uninstall: {}", name);

            // else
            // print info
        }
        Some(Commands::List {}) => {
            app.modules.all().into_iter()
                .filter(|m| m.is_installed())
                .for_each(|m| {
                    println!("{} [{}]", m.info().name, m.info().identifier);
                    println!("* installed: {}", m.is_installed());
                    println!()
                });

            for n in app.modules.names() {
                println!("{}", n);
            }
        }
        None => {}
    }
}
