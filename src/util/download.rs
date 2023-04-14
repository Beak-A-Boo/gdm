use std::{cmp, fs, io::Write, path::PathBuf};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use reqwest::Client;
use thiserror::Error;

use super::dirs;

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
    let client = Client::builder().user_agent(APP_USER_AGENT).build()?;
    Ok(client)
}

pub async fn download_file(url: String, local_path: String) -> Result<u64, DownloadError> {
    let download_dir = dirs::project_dirs().cache_dir().join("downloads");
    fs::create_dir_all(&download_dir)?;

    if PathBuf::from(&local_path).exists() {
        fs::remove_file(&local_path)?;
    }

    let mut rng = rand::thread_rng();
    let rand_int: u32 = rng.gen();

    let tmp_file = download_dir.join(rand_int.to_string());

    let client = make_client()?;
    let result = client.get(&url).send().await?;

    if result.status().is_success() {
        let total_size = result.content_length().unwrap_or(1024);

        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
        .progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", url));

        let mut file = fs::File::create(&tmp_file)?;
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
        fs::rename(tmp_file, local_path)?;
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
