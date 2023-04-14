mod project;
mod util;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use path_clean::PathClean;

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
    Set { version: String },

    #[clap(about = "Initialize a new project")]
    Init { path: Option<PathBuf> },

    // Engine {
    //     #[command(subcommand)]
    //     command: EngineCommands,
    // },
}

#[derive(Subcommand)]
enum EngineCommands {

}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upgrade => println!("Upgrade"),
        Commands::Set { version } => println!("Set: {}", version),
        Commands::Init { path } => {
            let actual_path = path.unwrap_or_else(|| PathBuf::from(".")).clean();
            
            match ProjectConfiguration::init(&actual_path).await {
                Ok(project) => {
                    println!("Successfully initialized new project: {}, Godot Engine v{}", &project.name.to_string(), project.config.version.to_string());
                },
                Err(e) => panic!("Error: {}", e),
            }
        },
        // Commands::Engine { command } => match command {
        //     EngineCommands::Help => println!("Engine Help"),
        //     EngineCommands::Upgrade => println!("Engine Upgrade"),
        //     EngineCommands::Set { version } => println!("Engine Set: {}", version),
        // },
    }
    // println!("{:?}", args);
}
