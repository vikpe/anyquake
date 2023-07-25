use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tokio;
use trauma::download::Download;
use trauma::downloader::DownloaderBuilder;

// use anyquake_core::modules::ModuleCollection;

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
async fn main() {
    let reqwest_rs = "https://github.com/seanmonstar/reqwest/archive/refs/tags/v0.11.9.zip";
    let downloads = vec![Download::try_from(reqwest_rs).unwrap()];
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from("output"))
        .build();
    downloader.download(&downloads).await;
}
