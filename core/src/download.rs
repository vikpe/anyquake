use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Result};
use futures_util::future::join_all;
use futures_util::StreamExt;

pub enum Dest<T> {
    File(T),
    Dir(T),
}

pub fn url_to_filename(url: &str) -> &str {
    url.split('/').last().unwrap()
}

pub async fn download_file(url: &str, dest: Dest<impl AsRef<Path>>) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    let file_path: String = match dest {
        Dest::File(path) => path.as_ref().to_str().unwrap().to_string(),
        Dest::Dir(path) => {
            let filename = url_to_filename(url);
            format!("{}/{}", path.as_ref().to_str().unwrap().to_string(), filename)
        }
    };

    let mut file = File::create(file_path)?;
    let content = response.bytes().await?;
    file.write_all(&content)?;

    Ok(())
}

pub async fn download_files(urls: Vec<(&str, &str)>) -> Result<()> {
    let tasks = urls.into_iter().map(|(url, file_path)| download_file(url, Dest::File(file_path)));
    let results = join_all(tasks).await;

    for result in results {
        if let Err(e) = result {
            eprintln!("Error while downloading file: {:?}", e);
        }
    }

    Ok(())
}

pub async fn download_files_to_dir(file_urls: Vec<&str>, dir_path: &str) -> Result<()> {
    let tasks = file_urls.into_iter().map(|url| download_file(url, Dest::Dir(dir_path)));
    let results = join_all(tasks).await;

    for result in results {
        if let Err(e) = result {
            eprintln!("Error while downloading file: {:?}", e);
        }
    }

    Ok(())
}

pub async fn download_file_with_progress(
    url: &str,
    file_path: &str,
    progress_callback: fn(u64, u64),
) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;

    let bytes_total = res.content_length().ok_or(anyhow!("Failed to get content length from '{}'", &url))?;
    let mut bytes_downloaded = 0;

    // download chunks
    let mut file = File::create(file_path).or(Err(anyhow!("Failed to create file '{}'", file_path)))?;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(anyhow!("Error while downloading file")))?;
        file.write_all(&chunk).or(Err(anyhow!("Error while writing to file")))?;
        bytes_downloaded = min(bytes_downloaded + (chunk.len() as u64), bytes_total);
        progress_callback(bytes_downloaded, bytes_total);
    }

    file.sync_all()?;

    Ok(())
}
