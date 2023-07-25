use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use reqwest::Client;

pub struct FileDownloader {
    client: Client,
}

impl FileDownloader {
    pub async fn download(&self, url: &str, path: &PathBuf) -> FileDownload {
        //download_file().await;
    }

    // pub async fn download_multiple() {
    //
    // }
}

pub struct FileDownload {
    pub url: String,
    pub path: PathBuf,
    pub bytes_total: u64,
    pub bytes_downloaded: u64,
    pub callback: fn(&FileDownload),
}

impl Default for FileDownload {
    fn default() -> Self {
        FileDownload {
            url: String::from(""),
            path: PathBuf::from(""),
            bytes_downloaded: 0,
            bytes_total: 0,
            callback: |f| {},
        }
    }
}

impl FileDownload {
    pub fn progress(&self) -> u8 {
        return 88;
    }

    pub fn is_complete(&self) -> bool {
        self.bytes_downloaded >= self.bytes_total
    }

    pub async fn start(&mut self, client: &Client, url: &str, path: &str, callback: &fn(&FileDownload)) -> Result<()> {
        let res = client
            .get(url)
            .send()
            .await
            .or(Err(anyhow!("Failed to GET from '{}'", &url)))?;

        self.bytes_total = res
            .content_length()
            .ok_or(anyhow!("Failed to get content length from '{}'", &url))?;

        // download chunks
        let mut file = File::create(path).or(Err(anyhow!("Failed to create file '{}'", path)))?;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(anyhow!("Error while downloading file")))?;
            file.write_all(&chunk).or(Err(anyhow!("Error while writing to file")))?;
            self.bytes_downloaded = min(self.bytes_downloaded + (chunk.len() as u64), self.bytes_total);
        }

        return Ok(());
    }
}

pub async fn download(client: &Client, file_download: &mut FileDownload, callback: &fn(&FileDownload)) -> Result<()> {
    let res = client
        .get(&file_download.url)
        .send()
        .await
        .or(Err(anyhow!("Failed to GET from '{}'", &file_download.url)))?;

    file_download.bytes_total = res
        .content_length()
        .ok_or(anyhow!("Failed to get content length from '{}'", file_download.url))?;

    // download chunks
    let mut file = File::create(&file_download.path).or(Err(anyhow!("Failed to create file '{}'", file_download.path.display())))?;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(anyhow!("Error while downloading file")))?;
        file.write_all(&chunk).or(Err(anyhow!("Error while writing to file")))?;
        file_download.bytes_downloaded = min(file_download.bytes_downloaded + (chunk.len() as u64), file_download.bytes_total);
    }

    Ok(())
}
