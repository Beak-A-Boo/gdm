use core::fmt;
use std::{path::PathBuf, fs, io, str};

use serde_derive::{Deserialize, Serialize};

use super::{engine::EngineVersion, Project};

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
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl ProjectConfiguration {
    pub fn new(version: String) -> ProjectConfiguration {
        ProjectConfiguration {
            download_source: EngineDownloadSource::GitHub,
            mono: true,
            version: EngineVersion::from_string(version), // TODO get latest version
        }
    }

    pub fn init(path: &PathBuf) -> Result<Project, io::Error> {
        match std::fs::metadata(path) {
            Ok(meta) if meta.is_file() => panic!("Path is a file, not a directory"),
            Ok(_) => { /* directory already exists */ },
            Err(_) => std::fs::create_dir_all(path).unwrap(),
        }
    
        let absolute_path = dunce::canonicalize(path).unwrap();

        let config_path = absolute_path.join("project.json");

        if config_path.exists() {
            panic!("Project already exists"); // TODO error handling
        }

        let dirname = absolute_path.file_name().unwrap().to_str().unwrap().to_string();
        println!("creating new project: {}", dirname);

        //TODO get latest version
        let version = "1.0.0-stable".to_string();

        let config = ProjectConfiguration::new(version);

        fs::write(config_path, serde_json::to_string_pretty(&config).unwrap())?;

        Ok(Project {
            name: dirname,
            path: absolute_path.clone(),
            config,
        })
    }
}
