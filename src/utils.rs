use crate::cli::Cli;
use crate::directory_operations::process_directory;
use crate::file_operations::process_file;
use anyhow::Result;
use std::path::Path;

pub fn separator() -> String {
    "-".repeat(80)
}

pub fn process_path(cli: &Cli, path: &Path) -> Result<(String, usize, usize, usize)> {
    if path.is_dir() {
        process_directory(cli, path)
    } else if path.is_file() {
        process_file(cli, path)
    } else {
        if cli.include_errors {
            eprintln!(
                "ERROR: Path '{}' is neither a file nor a directory",
                path.display()
            );
        }
        Ok((String::new(), 0, 0, 0))
    }
}

pub fn output_information(cli: &Cli) -> Result<String> {
    let mut total_chars = 0;
    let mut total_words = 0;
    let mut total_non_empty_lines = 0;
    let mut paths_str = String::new();
    let mut buf = String::new();

    for path in &cli.paths {
        match process_path(cli, path) {
            Ok((_, char_count, word_count, non_empty_line_count)) => {
                total_chars += char_count;
                total_words += word_count;
                total_non_empty_lines += non_empty_line_count;
                if !paths_str.is_empty() {
                    paths_str.push(' ');
                }
                paths_str.push_str(&path.display().to_string());
            }
            Err(e) => {
                if cli.include_errors {
                    eprintln!("ERROR gathering info for path {}: {}", path.display(), e);
                }
            }
        }
    }

    buf.push_str(&format!("Paths: {}\n", paths_str));
    buf.push_str(&format!("Total Characters: {}\n", total_chars));
    buf.push_str(&format!("Total Words: {}\n", total_words));
    buf.push_str(&format!("Total Non-Empty Lines: {}\n", total_non_empty_lines));

    Ok(buf)
}
