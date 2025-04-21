use crate::cli::Cli;
use crate::file_operations::process_file_content;
use crate::utils::separator;
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn process_entry_if_file(
    directory: &Path,
    include_errors: bool,
    output_information: bool,
    entry: DirEntry,
) -> Result<(String, usize, usize, usize)> {
    if entry.file_type().is_file() {
        match process_file_content(entry.path(), directory, output_information) {
            Ok((content, char_count, word_count, non_empty_line_count)) => {
                Ok((content, char_count, word_count, non_empty_line_count))
            }
            Err(e) => {
                if include_errors {
                    eprintln!("**{}:**", entry.path().display());
                    eprintln!("ERROR: Failed to process file: {}", e);
                    Ok((String::new(), 0, 0, 0))
                } else {
                    Ok((String::new(), 0, 0, 0))
                }
            }
        }
    } else {
        Ok((String::new(), 0, 0, 0))
    }
}

fn process_directory_contents(
    directory: &Path,
    max_depth: usize,
    include_errors: bool,
    output_information: bool,
) -> Result<(String, usize, usize, usize)> {
    let mut total_chars = 0;
    let mut total_words = 0;
    let mut total_non_empty_lines = 0;
    let mut buffer = String::new();
    let mut first_file_processed = true;

    for walk_entry_result in WalkDir::new(directory).max_depth(max_depth) {
        match walk_entry_result {
            Ok(entry) => {
                let entry_path: PathBuf = entry.path().to_path_buf();
                match process_entry_if_file(directory, include_errors, output_information, entry) {
                    Ok((content, char_count, word_count, non_empty_line_count)) => {
                        total_chars += char_count;
                        total_words += word_count;
                        total_non_empty_lines += non_empty_line_count;

                        if !content.is_empty() && !output_information {
                            if !first_file_processed {
                                buffer.push_str(&separator());
                                buffer.push('\n');
                            }
                            buffer.push_str(&content);
                            first_file_processed = false;
                        }
                    }
                    Err(e) => {
                        if include_errors {
                            eprintln!("ERROR processing entry {:?}: {}", entry_path, e);
                        }
                    }
                }
            }
            Err(e) => {
                if include_errors {
                    eprintln!(
                        "ERROR: Failed to read entry in {}: {}",
                        directory.display(),
                        e
                    );
                }
            }
        }
    }

    Ok((buffer, total_chars, total_words, total_non_empty_lines))
}

pub fn process_directory(cli: &Cli, path: &Path) -> Result<(String, usize, usize, usize)> {
    process_directory_contents(
        path,
        cli.max_depth,
        cli.include_errors,
        cli.output_information,
    )
}
