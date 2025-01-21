use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

use project::{engine::EngineVersion, versions};
use util::dirs;

use crate::project::config::ProjectConfiguration;

mod project;
mod util;
mod cli;

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

        #[clap(long, help = "Use Mono version of Godot Engine")]
        mono: bool,
    },
    #[clap(about = "Launch Godot Engine")]
    Run {
        path: Option<PathBuf>,

        #[clap(long, help = "Run the engine in console mode")]
        console: bool,
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
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    dotenvy::dotenv().ok();

    match cli.command {
        Commands::Upgrade { path } => {
            let dirs = dirs::init(path).await?;

            match project::Project::load(&dirs) {
                Ok(mut project) => {
                    println!(
                        "Found existing project: {}, Godot Engine v{}!",
                        &project.name.to_string(),
                        project.config.version.to_string()
                    );
                    let version = project.config.download_source.get_latest_version().await?;
                    println!("Found latest version: {}", version.to_string());
                    if version.to_string() != project.config.version.to_string() {
                        project.config.version = version;
                        project.save()?;
                        println!("Successfully upgraded Godot Engine to v{}", project.config.version.to_string());
                    } else {
                        println!("Project is already up to date!");
                    }

                    Ok(())

                }
                Err(_e) => {
                    let project = ProjectConfiguration::init(&dirs, false).await?;
                    println!(
                        "Successfully initialized new project: {}, Godot Engine v{}",
                        &project.name.to_string(),
                        project.config.version.to_string()
                    );

                    Ok(())
                },
            }
        },
        Commands::Set { version, path } => {
            let dirs = dirs::init(path).await?;
            let mut project = project::Project::load(&dirs)?;
            project.config.version = EngineVersion::from_string(version);
            project.save()?;
            println!(
                "Successfully set Godot Engine version to {}",
                project.config.version.to_string()
            );

            Ok(())
        }
        Commands::Init { path, mono } => {
            let dirs = dirs::init(path).await?;

            match project::Project::load(&dirs) {
                Ok(project) => {
                    println!(
                        "Found existing project: {}, Godot Engine v{}, aborting!",
                        &project.name.to_string(),
                        project.config.version.to_string()
                    );

                    Ok(())
                }
                Err(_e) => {
                    let project = ProjectConfiguration::init(&dirs, mono).await?;
                    println!(
                        "Successfully initialized new project: {}, Godot Engine v{}",
                        &project.name.to_string(),
                        project.config.version.to_string()
                    );

                    Ok(())
                },
            }
        }
        Commands::Run { path, console } => {
            let dirs = dirs::init(path).await?;

            let project = project::Project::load(&dirs)?;
            versions::ensure_version_installed(&project).await?;
            project.run(console).await?;

            Ok(())
        }
        Commands::Clean => {
            println!("Deleting all engine versions and cache...");
            let dirs = dirs::init_no_project().await?;
            let mut to_delete: Vec<PathBuf> = Vec::new();
            to_delete.push(dirs.cache_dir);
            to_delete.push(dirs.engines_install_dir);

            for path in &to_delete {
                if path.is_dir() {
                    fs::remove_dir_all(path)?;
                }
            }

            println!("Done!");
            Ok(())
        }
        // Commands::Engine { command } => match command {
        //     EngineCommands::Help => println!("Engine Help"),
        //     EngineCommands::Upgrade => println!("Engine Upgrade"),
        //     EngineCommands::Set { version } => println!("Engine Set: {}", version),
        // },
    }
    // println!("{:?}", args);
}
