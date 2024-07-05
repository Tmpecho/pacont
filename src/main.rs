use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The path to the directory to read files from
    path: PathBuf,
}

fn print_files_in_directory(directory: &Path) -> Result<()> {
    let mut stack = vec![PathBuf::from(directory)];
    let seperator = "-".repeat(80);

    while let Some(current_path) = stack.pop() {
        if current_path.is_dir() {
            for entry in fs::read_dir(current_path)? {
                let entry = entry?;
                stack.push(entry.path());
            }
        } else if current_path.is_file() {
            let relative_path = current_path.strip_prefix(directory).unwrap();
            println!("**{}**", relative_path.display());

            let mut file = fs::File::open(&current_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("{}", contents);
            println!("{}", seperator);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if !args.path.is_dir() {
        anyhow::bail!("Error: {} is not a valid directory", args.path.display());
    }

    print_files_in_directory(&args.path).context("Failed to read files in directory")
}