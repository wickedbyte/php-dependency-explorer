mod composer;

use crate::composer::{Composer, Php, VersionCommand};
use clap::{Parser, Subcommand};
use std::process::exit;

/// Provides a set of command line tools for exploring and extracting
/// information regarding a PHP project's dependencies.
#[derive(Parser, Debug)]
#[command(name = "PHP Dependency Explorer", version)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,
    #[arg(short, long, global = true)]
    php_path: Option<String>,
    #[arg(short, long, global = true)]
    composer_path: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds a new item
    Add {
        name: String,
        #[arg(short, long)]
        quantity: Option<u32>,
    },
    /// Lists existing items
    List {
        #[arg(short, long)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let php = Php::new(cli.php_path);
    let composer = Composer::new(cli.composer_path);

    // Verify that the required executables exist with the given paths
    check_version("PHP", &php, cli.verbose);
    check_version("Composer" ,&composer, cli.verbose);

    match &cli.command {
        Commands::Add { name, quantity } => {
            println!("Adding item: {}", name);
            if let Some(q) = quantity {
                println!("Quantity: {}", q);
            }
        }
        Commands::List { all } => {
            if *all {
                println!("Listing all items.");
            } else {
                composer.get_locked_dependencies();
            }
        }
    }
}

fn check_version(name: &str, versionable: &dyn VersionCommand, verbose: bool) {
    match versionable.version() {
        Ok(version_info) => {
            if verbose {
                println!("{version_info}");
            }
        }
        Err(error) => {
            eprintln!("ERROR: {name} Executable Not Found ({error})");
            exit(1);
        }
    }
}
