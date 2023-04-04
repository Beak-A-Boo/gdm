use std::path::PathBuf;

use self::config::ProjectConfiguration;

pub mod config;
pub mod engine;

pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub config: config::ProjectConfiguration,
}

impl Project {
    pub fn new(name: String, path: PathBuf) -> Project {
        //TODO load config
        let config = ProjectConfiguration::new("1.0.0-stable".to_string());

        Project {
            name,
            path,
            config,
        }
    }
}