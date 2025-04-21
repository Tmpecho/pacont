use crate::cli::Cli;
use anyhow::{Context, Result};
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn process_file_content(
    file_path: &Path,
    base_path: &Path,
    output_information: bool,
) -> Result<(String, usize, usize, usize)> {
    let relative_path = file_path
        .strip_prefix(base_path)
        .with_context(|| format!("Failed to strip prefix from {}", file_path.display()))?;
    let display_path =
        if base_path == Path::new("") || base_path == file_path.parent().unwrap_or(Path::new("")) {
            file_path
                .file_name()
                .unwrap_or(file_path.as_os_str())
                .to_string_lossy()
        } else {
            relative_path.display().to_string().into()
        };

    let mut file = fs::File::open(file_path)
        .with_context(|| format!("Failed to open file {}", file_path.display()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("Failed to read file contents of {}", file_path.display()))?;

    let char_count = contents.chars().count();
    let word_count = contents.split_whitespace().count();
    let non_empty_line_count = contents.lines().filter(|line| !line.trim().is_empty()).count();

    let mut output_buffer = String::new();
    if !output_information {
        output_buffer.push_str(&format!("**{}:**\n", display_path));
        output_buffer.push_str(&contents);
        output_buffer.push('\n');
    }

    Ok((output_buffer, char_count, word_count, non_empty_line_count))
}

pub fn process_file(cli: &Cli, path: &Path) -> Result<(String, usize, usize, usize)> {
    process_file_content(path, Path::new(""), cli.output_information)
}
