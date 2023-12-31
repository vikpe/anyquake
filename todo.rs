// CRATE: https://github.com/stanislav-tkach/os_info

// https://freshman.tech/snippets/go/cross-compile-go-programs/
// https://doc.rust-lang.org/reference/conditional-compilation.html
pub fn foo() {
    let machine_kind = if cfg!(unix) {
      "unix"
    } else if cfg!(windows) {
      "windows"
    } else {
      "unknown"
    };
    
    println!("I'm running on a {} machine!", machine_kind);
}

pub enum Status {
    Pending,
    // Queued,
    InProgress,
    // Running,
    // Paused,
    // Aborted,
    // Skipped(reason: String),
    // Canceled,
    Completed,
    Failed, // Failed(reason: String),
}


pub struct Task {
    pub url: String,
    pub file_path: String,
    pub bytes_total: u64,
    pub bytes_downloaded: u64,
    pub progress_callback: fn(u64, u64),
    pub status: Status,
}

impl Task {
    pub fn new(
        url: String,
        file_path: String,
        progress_callback: fn(u64, u64),
    ) -> Self {
        Self {
            url,
            file_path,
            progress_callback,
            bytes_total: 0,
            bytes_downloaded: 0,
            status: Status::Pending,
        }
    }

    pub fn start(&self) {
        self.status = Status::InProgress;
        let task = async move {
            download_file_with_progress(&self.url, &self.file_path, self.progress_callback).await
        };
        tokio::spawn(task);
    }
}


// ##########################################
use std::error::Error;
use std::fs::File;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::time::Duration;
use reqwest::Client;
use futures::future::join_all;

async fn download_file(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    let mut dest = File::create(file_path).await?;
    let content = response.bytes().await?;

    dest.write_all(&content).await?;

    Ok(())
}

async fn download_files_async(file_urls: Vec<(&str, &str)>) -> Result<(), Box<dyn Error>> {
    let tasks = file_urls.into_iter().map(|(url, file_path)| download_file(url, file_path));
    let results = join_all(tasks).await;

    for result in results {
        if let Err(e) = result {
            eprintln!("Error while downloading file: {:?}", e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // List of URLs and corresponding file paths
    let file_urls = vec![
        ("https://example.com/file1.txt", "file1.txt"),
        ("https://example.com/file2.txt", "file2.txt"),
        // Add more URLs and file paths here as needed
    ];

    if let Err(e) = download_files_async(file_urls).await {
        eprintln!("Error during async download: {:?}", e);
    }
}
