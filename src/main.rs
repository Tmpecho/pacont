use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Paths to directories or files to read
    paths: Vec<PathBuf>,

    /// Maximum recursion depth for directories
    #[clap(short, long, default_value = "10")]
    max_depth: usize,

    /// Include error messages in the output
    #[clap(short, long)]
    include_errors: bool,
}

fn separator() {
    println!("{}", "-".repeat(80));
}

/// Print the file path and the contents of a file
fn print_file(file_path: &Path, base_path: &Path) -> Result<()> {
    let relative_path = file_path.strip_prefix(base_path)
        .with_context(|| format!("Failed to strip prefix from {}", file_path.display()))?;
    let display_path = if base_path == Path::new("") {
        file_path
    } else {
        &base_path.join(relative_path)
    };

    let mut file = fs::File::open(file_path)
        .with_context(|| format!("Failed to open file {}", file_path.display()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("Failed to read file contents of {}", file_path.display()))?;

    println!("**{}:**", display_path.display());
    println!("{}", contents);
    println!();

    Ok(())
}

/// Process a directory entry if it is a file
fn process_if_file(directory: &Path, include_errors: bool, entry: DirEntry) {
    if entry.file_type().is_file() {
        match print_file(entry.path(), directory) {
            Ok(_) => separator(),
            Err(e) => {
                if include_errors {
                    println!("**{}:**", entry.path().display());
                    println!("ERROR: Failed to process file: {}", e);
                    separator();
                }
            }
        }
    }
}

/// Print the files in a directory and its subdirectories
fn print_files_in_directory(directory: &Path, max_depth: usize, include_errors: bool) -> Result<()> {
    for entry in WalkDir::new(directory).max_depth(max_depth) {
        match entry {
            Ok(entry) => {
                process_if_file(directory, include_errors, entry);
            }
            Err(e) => {
                if include_errors {
                    println!("ERROR: Failed to read entry in {}: {}", directory.display(), e);
                }
                continue;
            }
        }
    }
    Ok(())
}

/// Process a path, printing the contents of files and directories
fn process_path(cli: &Cli, path: &Path) {
    if path.is_dir() {
        if let Err(e) = print_files_in_directory(path, cli.max_depth, cli.include_errors) {
            if cli.include_errors {
                println!("ERROR: Failed to process directory {}: {}", path.display(), e);
            }
        }
    } else if path.is_file() {
        match print_file(path, Path::new("")) {
            Ok(_) => separator(),
            Err(e) => {
                if cli.include_errors {
                    println!("**{}:**", path.display());
                    println!("ERROR: Failed to process file: {}", e);
                    separator();
                }
            }
        }
    } else if cli.include_errors {
        println!("ERROR: Path '{}' is neither a file nor a directory", path.display());
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.paths.is_empty() {
        anyhow::bail!("No paths provided. Use --help for usage information.");
    }

    for path in &cli.paths {
        process_path(&cli, path);
    }

    Ok(())
}
