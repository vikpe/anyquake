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
            if let Some(module) = app.modules.by_id(String::from(name)) {
                println!("Install {}[{}]!", name, module.info().id);
            } else {
                println!("Module is {} not supported", name);
                println!("Supported modules: {}", app.modules.names().join(", "));
            }
        }
        Some(Commands::Uninstall { module: name }) => {
            if let Some(module) = app.modules.by_id(String::from(name)) {
                println!("Uninstall {}[{}]!", name, module.info().id);
            } else {
                println!("Module is {} not installed", name);
                let installed_modules: Vec<String> = app.modules.into_iter().filter(|m| m.is_installed()).map(|m| m.info().name).collect();

                println!("Installed modules: {}", installed_modules.join(", "));
            }
        }
        Some(Commands::List {}) => {
            app.modules.into_iter()
                .filter(|m| m.is_installed())
                .for_each(|m| {
                    println!("{} [{}]", m.info().name, m.info().id);
                    println!("* installed: {}", m.is_installed());
                });

            println!("\napp.modules.names()");
            for name in app.modules.names() {
                println!("{}", name);
            }

            println!("\napp.modules.by_id()");
            if let Some(module) = app.modules.by_id(String::from("ezquake")) {
                println!("{}", module.info().name)
            }
        }
        None => {}
    }
}
