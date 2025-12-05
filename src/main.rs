mod composer;

use crate::composer::{Composer, ComposerFiles, VersionCommand};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::exit;

/// Provides a set of command line tools for exploring and extracting
/// information regarding a PHP project's dependencies.
#[derive(Parser, Debug)]
#[command(name = "PHP Dependency Explorer", version)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[arg(short, long, global = true, conflicts_with = "composer")]
    docker: bool,

    #[arg(long = "no-cache", global = true)]
    no_cache: bool,

    #[arg(short, long, global = true)]
    php_path: Option<String>,

    #[arg(short, long, global = true)]
    composer: Option<String>,

    #[arg(long, global = true)]
    work_dir: Option<PathBuf>,

    #[arg(long, global = true)]
    cache_dir: Option<PathBuf>,

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

    let workdir = cli.work_dir.unwrap_or(std::env::current_dir().unwrap());
    let composer_files = ComposerFiles::from_path(workdir.as_path());
    match composer_files {
        Ok(composer_files) => {
            if cli.verbose {
                println!("{:?}", composer_files.json);
                println!("{:?}", composer_files.lock);
            }
        }
        Err(error) => {
            eprintln!(
                "ERROR: composer.json or composer.lock is missing, invalid, or cannot be read."
            );
            eprintln!("ERROR: {}", error.to_string());
            exit(1);
        }
    }

    let composer = Composer::new(if cli.docker {
        println!("Using Docker");
        let command = Some(format!(
            "docker run --rm -v {}:/app composer",
            workdir.display()
        ));
        command
    } else {
        println!("Using Local");
        cli.composer
    });

    // Verify that the required executables exist with the given paths
    check_version("Composer", &composer, cli.verbose);

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
