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
