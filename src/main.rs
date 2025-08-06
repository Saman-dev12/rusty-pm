mod commands;


use commands::init::init;
use commands::install::install;
use commands::remove::remove;
use commands::exec::exec_command;
use commands::run::run_script;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rusty-pm")]
#[command(about = "rusty-pm is a simple JS package manager made by Saman")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Install {
        package: String,
    },
    Remove {
	package: String,	
    },
  Run {
        script: String,
    },
  Exec {
        command: String,
    },
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Init => {
            println!("🔧 Initializing project...");
		init();
        }
        Commands::Install { package } => {
            println!("📦 Installing package: {}", package);
		install(package);
        },
	Commands::Remove { package } => {
	    println!("Removing package: {}" , package);
	    remove(package);
	}
	Commands::Run { script } => {
            println!("🚀 Running script: {}", script);
            run_script(script);
        }
	Commands::Exec { command } => {
            println!("Executing command: {}", command);
            exec_command(command);
        }
    }
}

