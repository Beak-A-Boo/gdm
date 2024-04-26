use std::{fs, path::PathBuf};

use anyhow::bail;

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
    pub fn load(path: &PathBuf) -> anyhow::Result<Project> {
        let absolute_path = dunce::canonicalize(path)?;

        let config_path = absolute_path.join("project.json");

        if !config_path.exists() {
            bail!("Project does not exist")// TODO error handling
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

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = self.path.join("project.json");

        let config = serde_json::to_string_pretty(&self.config)?;

        fs::write(config_path, config)?;

        Ok(())
    }

    pub async fn run(&self, console: bool) -> anyhow::Result<()> {
        let project_file = self.path.join("project.godot");
        if !project_file.exists() {
            println!("No project.godot file found, creating one...");
            fs::write(project_file, "").unwrap();
        }

        let dirs = dirs::project_dirs();
        let engine_name = self.config.get_engine_name();
        let engine_file_name = self.config.get_engine_file_name(console);

        let engine_path = dirs
            .data_local_dir()
            .join("engines")
            .join(&engine_name)
            .join(&engine_file_name);

        let mut command = std::process::Command::new(engine_path);
        command.arg("-e");
        command.current_dir(&self.path);
        command.spawn()?;

        Ok(())
    }
}
