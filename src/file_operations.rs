use std::fs;
use std::io::Read;
use std::path::Path;
use anyhow::{Context, Result};
use crate::cli::Cli;

pub fn print_file(file_path: &Path, base_path: &Path, output_information: bool) -> Result<(usize, usize)> {
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

pub fn process_file(cli: &Cli, path: &Path) -> Result<(usize, usize)> {
    print_file(path, Path::new(""), cli.output_information)
}