use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Paths to directories or files to read
    paths: Vec<PathBuf>,

    /// Maximum recursion depth for directories
    #[clap(short, long, default_value = "10")]
    max_depth: usize,
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

    println!("**{}:**", display_path.display());

    match fs::File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(e) => {
                        eprintln!("Warning: Failed to read line in {}: {}", file_path.display(), e);
                        continue;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to open file {}: {}", file_path.display(), e);
            return Ok(());
        }
    }

    println!();

    Ok(())
}

/// Print the files in a directory and its subdirectories
fn print_files_in_directory(directory: &Path, max_depth: usize) -> Result<()> {
    for entry in WalkDir::new(directory).max_depth(max_depth) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    if let Err(e) = print_file(entry.path(), directory) {
                        eprintln!("Warning: Failed to process file {}: {}", entry.path().display(), e);
                    }
                    println!("{}", "-".repeat(80));
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to read entry in {}: {}", directory.display(), e);
                continue;
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.paths.is_empty() {
        anyhow::bail!("No paths provided. Use --help for usage information.");
    }

    for path in &cli.paths {
        if path.is_dir() {
            if let Err(e) = print_files_in_directory(path, cli.max_depth) {
                eprintln!("Warning: Failed to process directory {}: {}", path.display(), e);
            }
        } else if path.is_file() {
            if let Err(e) = print_file(path, Path::new("")) {
                eprintln!("Warning: Failed to process file {}: {}", path.display(), e);
            }
        } else {
            eprintln!("Warning: Path '{}' is neither a file nor a directory", path.display());
        }
    }

    Ok(())
}
