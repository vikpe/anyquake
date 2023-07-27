enum Status {
    Pending,
    // Queued,
    InProgress, // Running,
    // Paused,
    // Aborted,
    // Skipped(reason: String),
    // Canceled,
    Completed,
    Failed, // Failed(reason: String),
}

struct Task {
    pub url: String,
    pub dest: PathBuf,
    pub bytes_total: u64,
    pub bytes_downloaded: u64,
    pub status: Status,
    pub on_change: fn(&Task),
     //pub fn skip()
     pub fn start()
     //pub fn stop()
     //pub fn resume()
     //pub fn restart()    
     //pub fn abort()    
}


use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use tokio::io::AsyncWriteExt;

// Function to download a file and write it to disk with progress reporting
async fn download_file_with_progress(
    url: &str,
    file_path: &str,
    progress_callback: impl Fn(u64, u64) + Send + Sync + 'static,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Send the request and obtain the response
    let response = client.get(url).send().await?;

    // Check if the request was successful (status code 2xx)
    if !response.status().is_success() {
        return Err(format!("Error: {}", response.status()).into());
    }

    // Get the total size of the file
    let total_size = response.content_length().unwrap_or(0);

    // Open a file for writing
    let mut file = File::create(file_path)?;

    // Buffer to store chunks of data
    let mut buffer = vec![0; 8192];

    // Track the downloaded bytes
    let mut downloaded_bytes = 0;

    // Read the response body and write it to the file
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;

        // Update the downloaded bytes
        downloaded_bytes += chunk.len() as u64;

        // Call the progress callback with current progress
        progress_callback(downloaded_bytes, total_size);
    }

    // Ensure everything is written to disk
    file.sync_all()?;

    Ok(())
}

// Example usage
#[tokio::main]
async fn main() {
    let url = "https://example.com/somefile.zip";
    let file_path = "/path/to/saved/file.zip";

    // Define the progress callback function
    let progress_callback = |downloaded: u64, total: u64| {
        println!("Downloaded: {}/{}", downloaded, total);
    };

    // Call the download function
    if let Err(err) = download_file_with_progress(url, file_path, progress_callback).await {
        eprintln!("Error: {:?}", err);
    } else {
        println!("Download complete!");
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
