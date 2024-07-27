use std::path::Path;
use anyhow::Result;
use walkdir::{DirEntry, WalkDir};
use crate::cli::Cli;
use crate::file_operations::print_file;
use crate::utils::separator;

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

pub fn process_directory(cli: &Cli, path: &Path) -> Result<(usize, usize)> {
    print_files_in_directory(path, cli.max_depth, cli.include_errors, cli.output_information)
}