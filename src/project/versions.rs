use crate::util::download::{self};

use super::engine::EngineVersion;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GithubReleaseResponse {
    pub tag_name: String,
}

pub async fn get_latest_version_from_github() -> Result<EngineVersion, download::DownloadError> {
    let url = "https://api.github.com/repos/godotengine/godot/releases/latest";
    let response = download::get_json::<GithubReleaseResponse>(url.to_string()).await?;

    Ok(EngineVersion::from_string(response.tag_name))
}

pub async fn download_from_github(
    path: String,
    filename: String,
    version: EngineVersion,
) -> Result<u64, download::DownloadError> {
    let url = format!(
        "https://github.com/godotengine/godot/releases/download/{}/{}",
        version, filename
    );
    download::download_file(url, path).await
}
