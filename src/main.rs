use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Paths to directories or files to read
    paths: Vec<PathBuf>,
}

/// Print the file path and the contents of a file
fn print_file(file_path: &Path, base_path: &Path) -> io::Result<()> {
    let relative_path = file_path.strip_prefix(base_path).unwrap_or(file_path);

    println!("**{}**", relative_path.display());

    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    println!("{}", contents);

    Ok(())
}

/// Print the files in a directory and its subdirectories
fn print_files_in_directory(directory: &Path) -> io::Result<()> {
    let mut stack = vec![PathBuf::from(directory)];

    while let Some(current_path) = stack.pop() {
        if current_path.is_dir() {
            for entry in fs::read_dir(current_path)? {
                let entry = entry?;
                stack.push(entry.path());
            }
        } else if current_path.is_file() {
            print_file(&current_path, directory)?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let separator = "-".repeat(80);

    if args.paths.is_empty() {
        anyhow::bail!("Error: No paths provided");
    }

    for path in &args.paths {
        if path.is_dir() {
            print_files_in_directory(path)?;
        } else if path.is_file() {
            print_file(path, path)?;
        } else {
            anyhow::bail!("Error: Path is not a file or directory");
        }

        println!("{}", separator);
    }

    Ok(())
}
