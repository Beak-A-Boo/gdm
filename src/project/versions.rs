use super::{Project, engine::EngineVersion};
use crate::project::config::EngineDownloadSource;
use crate::util::os::OS;
use crate::util::{archive, download};
use rootcause::prelude::ResultExt;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

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
    filename: &str,
    project: &Project,
) -> Result<u64, download::DownloadError> {
    let url = format!(
        "https://github.com/godotengine/godot/releases/download/{version}/{filename}",
        version = project.config.version,
    );
    download::download_file(url, path, &project.dirs).await
}

pub async fn ensure_version_installed(project: &Project) -> rootcause::Result<()> {
    let config = &project.config;
    let engine_name = config.get_engine_name();
    let engine_file_name = config.get_engine_file_name(false);

    let dirs = &project.dirs;

    let engine_dir = dirs.engines_install_dir.join(&engine_name);
    let engine_file = engine_dir.join(&engine_file_name);

    if !engine_file.exists() {
        println!("Could not find matching version of Godot engine locally, downloading...");

        let zip_file_name = format!("{engine_name}.zip");

        let zip_file_path = dirs.cache_dir.join("engines").join(&zip_file_name);

        let os = OS::current();

        match project.config.download_source {
            EngineDownloadSource::GitHub => {
                let zip_file_name_remote = match (os, config.mono) {
                    (OS::Windows, false) => format!("{engine_name}.exe.zip"),
                    (OS::MacOS, _) => format!("{engine_name}.universal.zip"),
                    (_, _) => format!("{engine_name}.zip"),
                };

                download_from_github(&zip_file_path, &zip_file_name_remote, project)
                    .await
                    .context_with(|| {
                        format!(
                            "path: {zip_path}, remote: {zip_file_name_remote}",
                            zip_path = zip_file_path.display()
                        )
                    })?;
            }
            EngineDownloadSource::GodotEngineOrg => {
                let version_major = project.config.version.version_string.clone();
                let version_build = project
                    .config
                    .version
                    .build_string
                    .as_deref()
                    .unwrap_or("stable");

                let os_arch_slug = OS::current()
                    .get_os_string(project.config.mono)?
                    .to_string();
                let zip_file_name_remote = match (os, config.mono) {
                    (OS::Windows, false) => format!("{os_arch_slug}.exe.zip"),
                    (OS::MacOS, _) => format!("{os_arch_slug}.universal.zip"),
                    (_, _) => format!("{os_arch_slug}.zip"),
                };

                let os_arch_name = format!(
                    "{os_name}.{os_bits}",
                    os_name = os.get_os_name(),
                    os_bits = os.architecture().get_bits()
                );

                let url = format!(
                    "https://downloads.godotengine.org/?version={version_major}&flavor={version_build}&slug={zip_file_name_remote}&platform={os_arch_name}"
                );
                download::download_file(url, &zip_file_path, &project.dirs).await?;
            }
        }

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

        println!("Successfully installed Godot engine version {engine_name}")
    }

    Ok(())
}
