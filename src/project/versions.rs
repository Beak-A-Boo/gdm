use std::{fs, path::PathBuf};

use crate::util::{
    dirs,
    download::{self},
};

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
    path: &PathBuf,
    filename: String,
    version: EngineVersion,
) -> Result<u64, download::DownloadError> {
    let url = format!(
        "https://github.com/godotengine/godot/releases/download/{}/{}",
        version, filename
    );
    download::download_file(url, path).await
}

pub async fn ensure_version_installed(
    version: EngineVersion,
    mono: bool,
) -> Result<(), download::DownloadError> {
    let mut os_string = "win64.exe";
    if mono {
        os_string = "mono_win64";
    }
    let dirs = dirs::project_dirs();
    let mut engine_name = version.to_string();
    if mono {
        engine_name.push_str("-mono");
    }
    let engine_dir = dirs.data_local_dir().join("engines").join(&engine_name);

    let filename = format!("Godot_v{}_{}.zip", version.to_string(), os_string);
    let filepath = engine_dir.join(&filename);

    if !filepath.exists() {
        println!("Could not find matching version of Godot engine locally, downloading...");
        fs::create_dir_all(&engine_dir)?;
        download_from_github(&filepath, filename, version).await?;
        //TODO unzip
        println!(
            "Successfully installed Godot engine version {}",
            &engine_name
        )
    }

    Ok(())
}
