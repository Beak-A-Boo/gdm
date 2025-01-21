use clap::{Parser, Subcommand};
use const_format::concatcp;
use gdm::project::config::ProjectConfiguration;
use gdm::project::engine::EngineVersion;
use gdm::project::versions;
use gdm::util::dirs;
use gdm::{built_info, project};
use std::fs;
use std::path::PathBuf;

const ABOUT: &str = concatcp!(
    built_info::PKG_DESCRIPTION,
    "\n - ",
    built_info::PKG_HOMEPAGE,
    "\ngit: ",
    built_info::GIT_VERSION.unwrap()
);

#[derive(Parser)]
#[command(author = gdm::built_info::PKG_AUTHORS, version = gdm::VERSION, about = ABOUT)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Upgrade Godot Engine to the latest version")]
    Upgrade { path: Option<PathBuf> },
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
                        "Found existing project: {name}, Godot Engine v{engine_version}!",
                        name = &project.name,
                        engine_version = project.config.version
                    );
                    let version = project.config.download_source.get_latest_version().await?;
                    println!("Found latest version: {version}");
                    if version != project.config.version {
                        project.config.version = version;
                        project.save()?;
                        println!(
                            "Successfully upgraded Godot Engine to v{engine_version}",
                            engine_version = project.config.version
                        );
                    } else {
                        println!("Project is already up to date!");
                    }

                    Ok(())
                }
                Err(_e) => {
                    let project = ProjectConfiguration::init(&dirs, false).await?;
                    println!(
                        "Successfully initialized new project: {name}, Godot Engine v{engine_version}",
                        name = &project.name,
                        engine_version = project.config.version
                    );

                    Ok(())
                }
            }
        }
        Commands::Set { version, path } => {
            let dirs = dirs::init(path).await?;
            let mut project = project::Project::load(&dirs)?;
            project.config.version = EngineVersion::from_string(version);
            project.save()?;
            println!(
                "Successfully set Godot Engine version to {engine_version}",
                engine_version = project.config.version
            );

            Ok(())
        }
        Commands::Init { path, mono } => {
            let dirs = dirs::init(path).await?;

            match project::Project::load(&dirs) {
                Ok(project) => {
                    println!(
                        "Found existing project: {name}, Godot Engine v{engine_version}, aborting!",
                        name = &project.name,
                        engine_version = project.config.version
                    );

                    Ok(())
                }
                Err(_e) => {
                    let project = ProjectConfiguration::init(&dirs, mono).await?;
                    println!(
                        "Successfully initialized new project: {name}, Godot Engine v{engine_version}",
                        name = &project.name,
                        engine_version = project.config.version
                    );

                    Ok(())
                }
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

            for path in [dirs.cache_dir, dirs.engines_install_dir] {
                if path.is_dir() {
                    fs::remove_dir_all(path)?;
                }
            }

            println!("Done!");
            Ok(())
        } // Commands::Engine { command } => match command {
          //     EngineCommands::Help => println!("Engine Help"),
          //     EngineCommands::Upgrade => println!("Engine Upgrade"),
          //     EngineCommands::Set { version } => println!("Engine Set: {}", version),
          // },
    }
    // println!("{:?}", args);
}
