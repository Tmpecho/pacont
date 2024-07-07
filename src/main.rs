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

    /// Get number of characters and words of output (useful if output could be too long)
    #[clap(short, long)]
    output_information: bool,
}

fn separator() {
    println!("{}", "-".repeat(80));
}

/// Print the file path and the contents of a file
fn print_file(file_path: &Path, base_path: &Path, output_information: bool) -> Result<(usize, usize)> {
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

    if !output_information {
        println!("**{}:**", display_path.display());
        println!("{}", contents);
        println!();
    }

    let char_count = contents.chars().count();
    let word_count = contents.split_whitespace().count();

    Ok((char_count, word_count))
}

/// Process a directory entry if it is a file
fn process_if_file(directory: &Path, include_errors: bool, output_information: bool, entry: DirEntry) -> Result<(usize, usize)> {
    if entry.file_type().is_file() {
        match print_file(entry.path(), directory, output_information) {
            Ok((char_count, word_count)) => {
                if !output_information {
                    separator();
                }
                return Ok((char_count, word_count));
            },
            Err(e) => {
                if include_errors {
                    println!("**{}:**", entry.path().display());
                    println!("ERROR: Failed to process file: {}", e);
                    separator();
                }
            }
        }
    }
    Ok((0, 0))
}

/// Print the files in a directory and its subdirectories
fn print_files_in_directory(directory: &Path, max_depth: usize, include_errors: bool, output_information: bool) -> Result<(usize, usize)> {
    let mut total_chars = 0;
    let mut total_words = 0;

    for entry in WalkDir::new(directory).max_depth(max_depth) {
        match entry {
            Ok(entry) => {
                let (char_count, word_count) = process_if_file(directory, include_errors, output_information, entry)?;
                total_chars += char_count;
                total_words += word_count;
            }
            Err(e) => {
                if include_errors {
                    println!("ERROR: Failed to read entry in {}: {}", directory.display(), e);
                }
                continue;
            }
        }
    }

    Ok((total_chars, total_words))
}

fn process_directory(cli: &Cli, path: &Path) -> Result<(usize, usize)> {
    print_files_in_directory(path, cli.max_depth, cli.include_errors, cli.output_information)
}

fn process_file(cli: &Cli, path: &Path) -> Result<(usize, usize)> {
    print_file(path, Path::new(""), cli.output_information)
}

fn process_path(cli: &Cli, path: &Path) -> Result<(usize, usize)> {
    if path.is_dir() {
        process_directory(cli, path)
    } else if path.is_file() {
        process_file(cli, path)
    } else {
        if cli.include_errors {
            println!("ERROR: Path '{}' is neither a file nor a directory", path.display());
        }
        Ok((0, 0))
    }
}

fn print_output_information(cli: &Cli) -> Result<()> {
    let mut total_chars = 0;
    let mut total_words = 0;
    let mut paths = String::new();

    for path in &cli.paths {
        let (char_count, word_count) = process_path(cli, path)?;
        total_chars += char_count;
        total_words += word_count;
        if !paths.is_empty() {
            paths.push(' ');
        }
        paths.push_str(&path.display().to_string());
    }

    println!("Paths: {}", paths);
    println!("Total Characters: {}", total_chars);
    println!("Total Words: {}", total_words);

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.paths.is_empty() {
        anyhow::bail!("No paths provided. Use --help for usage information.");
    }

    if cli.output_information {
        print_output_information(&cli)?;
    } else {
        for path in &cli.paths {
            process_path(&cli, path)?;
        }
    }

    Ok(())
}