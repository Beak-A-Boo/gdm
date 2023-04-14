use std::{fs, cmp, io::Write};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use thiserror::Error;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("HTTP Error")]
    HttpError(#[from] reqwest::Error),
    #[error("Unknown Error")]
    Unknown,
}

pub fn make_client() -> Result<Client, DownloadError> {
    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    Ok(client)
}

pub async fn download_file(url: String, local_path: String) -> Result<u64, DownloadError> {
    let client = make_client()?;
    let result = client.get(&url).send().await?;

    if result.status().is_success() {
        let total_size = result.content_length().unwrap_or(1024);

        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
        .progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", url));

        let mut file = fs::File::create(local_path)?;
        let mut downloaded: u64 = 0;
        let mut stream = result.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;
            pb.set_position(cmp::min(downloaded, total_size));
        }
        pb.set_position(total_size);
        pb.finish_with_message(format!("Downloaded {} bytes", downloaded));
        Ok(downloaded)
    } else {
        Err(DownloadError::Unknown) //TODO status code error?
    }
}

pub async fn get_json<T: serde::de::DeserializeOwned>(url: String) -> Result<T, DownloadError> {
    let client = make_client()?;
    let result = client.get(&url).send().await?;
    if result.status().is_success() {
        Ok(result.json::<T>().await?)
    } else {
        Err(DownloadError::Unknown) //TODO status code error?
    }
}