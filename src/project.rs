use std::path::PathBuf;

use self::{config::{ProjectConfiguration, EngineDownloadSource}, engine::EngineVersion};

pub mod config;
pub mod engine;
pub mod versions;

pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub config: config::ProjectConfiguration,
}

impl Project {
    pub async fn new(name: String, path: PathBuf) -> Result<Project, Box<dyn std::error::Error>> {
        //TODO load config
        let config = ProjectConfiguration::new(EngineVersion::from_string("1.0.0-stable".to_string()), EngineDownloadSource::GitHub).await?;

        Ok(Project {
            name,
            path,
            config,
        })
    }
}