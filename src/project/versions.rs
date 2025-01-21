use std::{fs, path::PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::util::os::OS;
use crate::util::{archive, download};

use super::{engine::EngineVersion, Project};

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
    project: &Project,
) -> Result<u64, download::DownloadError> {
    let url = format!(
        "https://github.com/godotengine/godot/releases/download/{}/{}",
        &project.config.version, filename
    );
    download::download_file(url, path, &project.dirs).await
}

pub async fn ensure_version_installed(project: &Project) -> anyhow::Result<()> {
    let config = &project.config;
    let engine_name = config.get_engine_name();
    let engine_file_name = config.get_engine_file_name(false);

    let dirs = &project.dirs;

    let engine_dir = dirs.engines_install_dir.join(&engine_name);
    let engine_file = engine_dir.join(&engine_file_name);

    if !engine_file.exists() {
        println!("Could not find matching version of Godot engine locally, downloading...");

        let zip_file_name = format!("{}.zip", &engine_name);

        let zip_file_name_remote = if OS::current().is_windows() && !config.mono {
            format!("{}.exe.zip", &engine_name)
        } else {
            format!("{}.zip", &engine_name)
        };
        let zip_file_path = dirs.cache_dir.join("engines").join(&zip_file_name);

        download_from_github(&zip_file_path, zip_file_name_remote, project).await?;

        println!("Extracting archive...");
        archive::extract(&zip_file_path, &engine_dir, Some(true))?;
        for entry in [
            config.get_engine_file_name(false),
            config.get_engine_file_name(true),
        ] {
            let entry_path = engine_dir.join(&entry);
            if entry_path.exists() {
                //todo set executable bit
                OS::current().set_executable(&entry_path)?;
            }
        }

        println!("Reclaiming disk space...");
        if dirs.download_dir.is_dir() {
            fs::remove_dir_all(&dirs.download_dir)?;
        }

        println!(
            "Successfully installed Godot engine version {}",
            &engine_name
        )
    }

    Ok(())
}
