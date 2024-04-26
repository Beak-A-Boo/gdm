use std::{fs, path::PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::util::{archive, dirs, download};

use super::{config::ProjectConfiguration, engine::EngineVersion};

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
    version: &EngineVersion,
) -> Result<u64, download::DownloadError> {
    let url = format!(
        "https://github.com/godotengine/godot/releases/download/{}/{}",
        version, filename
    );
    download::download_file(url, path).await
}

pub async fn ensure_version_installed(
    config: &ProjectConfiguration,
) -> Result<(), download::DownloadError> {
    let engine_name = config.get_engine_name();
    let engine_file_name = config.get_engine_file_name(false);

    let dirs = dirs::project_dirs();

    let engine_dir = dirs.data_local_dir().join("engines").join(&engine_name);
    let engine_file = engine_dir.join(&engine_file_name);

    if !engine_file.exists() {
        println!("Could not find matching version of Godot engine locally, downloading...");

        let zip_file_name = format!("{}.zip", &engine_name);
        let zip_file_name_remote = format!("{}.zip", &engine_file_name);
        let zip_file_path = dirs.cache_dir().join("engines").join(&zip_file_name);

        download_from_github(&zip_file_path, zip_file_name_remote, &config.to_owned().version).await?;

        println!("Extracting archive...");
        archive::extract(&zip_file_path, &engine_dir, Some(true))?;

        println!("Reclaiming disk space...");
        let tmp_dir = dirs::project_dirs().cache_dir().join(".temp");
        if tmp_dir.is_dir() {
            fs::remove_dir_all(tmp_dir)?;
        }

        println!(
            "Successfully installed Godot engine version {}",
            &engine_name
        )
    }

    Ok(())
}
