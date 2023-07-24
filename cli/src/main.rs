use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();
    let modules = ModuleCollection::new();

    match &cli.command {
        Some(Commands::Install { module: name }) => {
            if let Some(module) = modules.by_id(String::from(name)) {
                println!("Install {}[{}]!", name, module.info().id);
                match module.install() {
                    Err(e) => {
                        println!("{:?}", e);
                    }
                    Ok(m) => {
                        println!("{:?}", m);
                    }
                }
            } else {
                println!("Module is {} not supported", name);
                println!("Supported modules: {}", modules.names().join(", "));
            }
        }
        Some(Commands::Uninstall { module: name }) => {
            if let Some(module) = modules.by_id(String::from(name)) {
                println!("Uninstall {}[{}]!", name, module.info().id);
            } else {
                println!("Module is {} not installed", name);
                let installed_modules: Vec<String> = modules.into_iter().filter(|m| m.is_installed()).map(|m| m.info().name).collect();

                println!("Installed modules: {}", installed_modules.join(", "));
            }
        }
        Some(Commands::List {}) => {
            modules.into_iter()
                .filter(|m| m.is_installed())
                .for_each(|m| {
                    println!("{} [{}]", m.info().name, m.info().id);
                    println!("* installed: {}", m.is_installed());
                });

            println!("\napp.modules.names()");
            for name in modules.names() {
                println!("{}", name);
            }

            println!("\napp.modules.by_id()");
            if let Some(module) = modules.by_id(String::from("ezquake")) {
                println!("{}", module.info().name)
            }
        }
        None => {}
    }
}
