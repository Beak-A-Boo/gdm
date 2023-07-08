mod project;
mod util;
use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

use project::{engine::EngineVersion, versions};
use util::dirs;

use crate::project::config::{ProjectConfiguration, EngineDownloadSource};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Upgrade Godot Engine to the latest version")]
    Upgrade {
        path: Option<PathBuf>,
    },
    #[clap()]
    Set {
        version: String,
        path: Option<PathBuf>,
    },

    #[clap(about = "Initialize a new project")]
    Init {
        path: Option<PathBuf>,
    },
    #[clap(about = "Launch Godot Engine")]
    Run {
        path: Option<PathBuf>,
    },
    #[clap(about = "Uninstall all engine versions and clear download cache")]
    Clean,
    // Engine {
    //     #[command(subcommand)]
    //     command: EngineCommands,
    // },
}

#[derive(Subcommand)]
enum EngineCommands {}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upgrade { path } => {
            let actual_path = dirs::get_actual_path(path);
            match project::Project::load(&actual_path) {
                Ok(mut project) => {
                    println!(
                        "Found existing project: {}, Godot Engine v{}!",
                        &project.name.to_string(),
                        project.config.version.to_string()
                    );
                    match project.config.download_source.get_latest_version().await {
                        Ok(version) => {
                            println!("Found latest version: {}", version.to_string());
                            if version.to_string() != project.config.version.to_string() {
                                project.config.version = version;
                                project.save().unwrap();
                                println!("Successfully upgraded Godot Engine to v{}", project.config.version.to_string());
                            } else {
                                println!("Project is already up to date!");
                            }
                        },
                        Err(e) => panic!("Error: {}", e),
                    }

                }
                Err(_e) => match ProjectConfiguration::init(&actual_path).await {
                    Ok(project) => {
                        println!(
                            "Successfully initialized new project: {}, Godot Engine v{}",
                            &project.name.to_string(),
                            project.config.version.to_string()
                        );
                    }
                    Err(e) => panic!("Error: {}", e),
                },
            }
        },
        Commands::Set { version, path } => {
            let actual_path = dirs::get_actual_path(path);
            match project::Project::load(&actual_path) {
                Ok(mut project) => {
                    project.config.version = EngineVersion::from_string(version);
                    project.save().unwrap();
                    println!(
                        "Successfully set Godot Engine version to {}",
                        project.config.version.to_string()
                    );
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
        Commands::Init { path } => {
            let actual_path = dirs::get_actual_path(path);

            match project::Project::load(&actual_path) {
                Ok(project) => {
                    println!(
                        "Found existing project: {}, Godot Engine v{}, aborting!",
                        &project.name.to_string(),
                        project.config.version.to_string()
                    );
                }
                Err(_e) => match ProjectConfiguration::init(&actual_path).await {
                    Ok(project) => {
                        println!(
                            "Successfully initialized new project: {}, Godot Engine v{}",
                            &project.name.to_string(),
                            project.config.version.to_string()
                        );
                    }
                    Err(e) => panic!("Error: {}", e),
                },
            }
        }
        Commands::Run { path } => {
            let actual_path = dirs::get_actual_path(path);

            match project::Project::load(&actual_path) {
                Ok(project) => {
                    versions::ensure_version_installed(&project.config).await.unwrap();
                    project.run().await.unwrap();

                }
                Err(e) => panic!("Error: {}", e),
            }
        }
        Commands::Clean => {
            println!("Deleting all engine versions and cache...");
            let project_dirs = dirs::project_dirs();
            let mut to_delete: Vec<PathBuf> = Vec::new();
            to_delete.push(project_dirs.cache_dir().to_path_buf());
            to_delete.push(project_dirs.data_local_dir().join("engines"));

            for path in &to_delete {
                if path.is_dir() {
                    fs::remove_dir_all(path).unwrap();
                }
            }

            println!("Done!");
        }
        // Commands::Engine { command } => match command {
        //     EngineCommands::Help => println!("Engine Help"),
        //     EngineCommands::Upgrade => println!("Engine Upgrade"),
        //     EngineCommands::Set { version } => println!("Engine Set: {}", version),
        // },
    }
    // println!("{:?}", args);
}
