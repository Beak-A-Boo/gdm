use std::{fs, path::PathBuf};

pub mod config;
pub mod engine;
pub mod versions;

pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub config: config::ProjectConfiguration,
}

impl Project {
    pub fn load(path: &PathBuf) -> Result<Project, Box<dyn std::error::Error>> {
        let absolute_path = dunce::canonicalize(path)?;

        let config_path = absolute_path.join("project.json");

        if !config_path.exists() {
            panic!("Project does not exist"); // TODO error handling
        }

        let project_name = absolute_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let config = serde_json::from_str(&fs::read_to_string(config_path)?)?;

        Ok(Project {
            name: project_name,
            path: path.clone(),
            config,
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.path.join("project.json");

        let config = serde_json::to_string_pretty(&self.config)?;

        fs::write(config_path, config)?;

        Ok(())
    }
}
