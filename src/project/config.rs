use std::{path::PathBuf, fs, io};

use serde_derive::{Deserialize, Serialize};

use super::{engine::EngineVersion, Project};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectConfiguration {

    pub download_source: EngineDownloadSource,
    pub version: EngineVersion,
    pub mono: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum EngineDownloadSource {
    GitHub,
    // TODO add tuxfamily
    // https://downloads.tuxfamily.org/godotengine/
}

impl ProjectConfiguration {
    pub fn new(version: String) -> ProjectConfiguration {
        return ProjectConfiguration {
            download_source: EngineDownloadSource::GitHub,
            mono: true,
            version: EngineVersion::from_string(version), // TODO get latest version
        };
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

        return Ok(Project {
            name: dirname,
            path: absolute_path.clone(),
            config,
        });
    }
}
