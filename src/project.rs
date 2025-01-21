use std::{fs, path::PathBuf};

use anyhow::bail;

use crate::util::dirs::Dirs;

pub mod config;
pub mod engine;
pub mod versions;

pub struct Project {
    pub name: String,
    pub config: config::ProjectConfiguration,
    pub dirs: Dirs,
}

impl Project {
    pub fn load(dirs: &Dirs) -> anyhow::Result<Project> {
        let project_absolute_path = dunce::canonicalize(&dirs.project_dir)?;

        let config_path = project_absolute_path.join("project.json");

        if !config_path.exists() {
            bail!("Project does not exist") // TODO error handling
        }

        let project_name = project_absolute_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let config = serde_json::from_str(&fs::read_to_string(config_path)?)?;

        Ok(Project {
            name: project_name,
            dirs: dirs.clone(),
            config,
        })
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = self.path().join("project.json");

        let config = serde_json::to_string_pretty(&self.config)?;

        fs::write(config_path, config)?;

        Ok(())
    }

    pub fn path(&self) -> PathBuf {
        self.dirs.absolute_project_dir.clone()
    }

    pub async fn run(&self, console: bool) -> anyhow::Result<()> {
        let project_file = self.path().join("project.godot");
        if !project_file.exists() {
            println!("No project.godot file found, creating one...");
            fs::write(project_file, "")?;
        }

        let engine_name = self.config.get_engine_name();
        let engine_file_name = self.config.get_engine_file_name(console);

        let engine_path = &self
            .dirs
            .engines_install_dir
            .join(&engine_name)
            .join(&engine_file_name);

        let mut command = std::process::Command::new(engine_path);
        command.arg("-e");
        command.current_dir(self.path());
        command.spawn()?;

        Ok(())
    }
}
