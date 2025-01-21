use core::fmt;
use std::str;

use crate::util::dirs::Dirs;
use crate::util::os::OS;
use anyhow::bail;
use serde_derive::{Deserialize, Serialize};

use super::{engine::EngineVersion, versions, Project};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectConfiguration {
    pub download_source: EngineDownloadSource,
    pub version: EngineVersion,
    pub mono: bool,
}

#[derive(Debug, serde_with::DeserializeFromStr)]
pub enum EngineDownloadSource {
    GitHub,
    // TODO add tuxfamily
    // https://downloads.tuxfamily.org/godotengine/
}

impl fmt::Display for EngineDownloadSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineDownloadSource::GitHub => write!(f, "github"),
        }
    }
}

impl str::FromStr for EngineDownloadSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "github" => Ok(EngineDownloadSource::GitHub),
            _ => Err(format!("Invalid download source: {}", s)),
        }
    }
}

impl serde::ser::Serialize for EngineDownloadSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl EngineDownloadSource {
    pub async fn get_latest_version(&self) -> anyhow::Result<EngineVersion> {
        match self {
            EngineDownloadSource::GitHub => Ok(versions::get_latest_version_from_github().await?),
        }
    }
}

impl ProjectConfiguration {
    pub async fn new(
        version: EngineVersion,
        download_source: EngineDownloadSource,
        mono: bool,
    ) -> anyhow::Result<ProjectConfiguration> {
        Ok(ProjectConfiguration {
            download_source,
            mono,
            version,
        })
    }

    pub async fn init(dirs: &Dirs, mono: bool) -> anyhow::Result<Project> {
        match std::fs::metadata(&dirs.absolute_project_dir) {
            Ok(meta) if meta.is_file() => bail!(
                "Path is a file, not a directory: {}",
                dirs.absolute_project_dir.display()
            ),
            Ok(_) => { /* directory already exists */ }
            Err(_) => std::fs::create_dir_all(&dirs.project_dir)?,
        }

        let config_path = dirs.absolute_project_dir.join("project.json");

        if config_path.exists() {
            bail!("Project already exists");
        }

        let directory_name = dirs
            .absolute_project_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let source = EngineDownloadSource::GitHub;
        let version = source.get_latest_version().await?;

        let config = ProjectConfiguration::new(version, source, mono).await?; // TODO error handling

        let project = Project {
            name: directory_name,
            config,
            dirs: dirs.clone(),
        };

        project.save()?;

        Ok(project)
    }

    pub fn get_engine_name(&self) -> String {
        let os = OS::current();
        let os_string = os.get_os_string(self.mono).expect("Invalid OS");
        let engine_name = self.version.to_string().clone();

        format!("Godot_v{}_{}", engine_name, os_string)
    }

    pub fn get_engine_file_name(&self, console: bool) -> String {
        let mut engine_name = self.get_engine_name();
        if console {
            engine_name.push_str("_console");
        }
        if OS::current().is_windows() {
            engine_name.push_str(".exe")
        }

        engine_name
    }
}
