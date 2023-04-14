mod project;
mod util;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use project::{engine::EngineVersion, versions};
use util::dirs;

use crate::project::config::ProjectConfiguration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Upgrade,
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
    }, // Engine {
       //     #[command(subcommand)]
       //     command: EngineCommands,
       // },
}

#[derive(Subcommand)]
enum EngineCommands {}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // empty strings for qualifier and org name are fine

    match cli.command {
        Commands::Upgrade => println!("Upgrade"),
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
        },
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
                    versions::ensure_version_installed(project.config.version, project.config.mono)
                        .await
                        .unwrap();
                    //TODO launch godot engine
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
        // Commands::Engine { command } => match command {
        //     EngineCommands::Help => println!("Engine Help"),
        //     EngineCommands::Upgrade => println!("Engine Upgrade"),
        //     EngineCommands::Set { version } => println!("Engine Set: {}", version),
        // },
    }
    // println!("{:?}", args);
}
