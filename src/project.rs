use std::{fs, path::PathBuf};

use crate::util::dirs;

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
            return Err("Project does not exist".into()); // TODO error handling
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

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let project_file = self.path.join("project.godot");
        if !project_file.exists() {
            println!("No project.godot file found, creating one...");
            fs::write(project_file, "").unwrap();
        }

        let dirs = dirs::project_dirs();
        let engine_name = self.config.get_engine_name();

        //TODO os-dependent engine path
        let engine_path = dirs
            .data_local_dir()
            .join("engines")
            .join(&engine_name)
            .join(format!("{}.exe", &engine_name));

        let mut command = std::process::Command::new(engine_path);
        command.arg("-e");
        command.current_dir(&self.path);
        command.spawn()?;

        Ok(())
    }
}
