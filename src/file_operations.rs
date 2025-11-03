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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_process_file_content_basic() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "Hello\nWorld").unwrap();

        let (content, chars, words, lines) =
            process_file_content(&file_path, temp_dir.path(), false).unwrap();

        assert!(content.contains("**test.txt:**"));
        assert!(content.contains("Hello\nWorld"));
        assert_eq!(chars, 12); // "Hello\nWorld\n" = 12 characters
        assert_eq!(words, 2);
        assert_eq!(lines, 2);
    }

    #[test]
    fn test_process_file_content_with_empty_lines() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "Line1\n\nLine2\n\n\nLine3").unwrap();

        let (_content, _chars, _words, lines) =
            process_file_content(&file_path, temp_dir.path(), false).unwrap();

        assert_eq!(lines, 3); // Only non-empty lines
    }

    #[test]
    fn test_process_file_content_output_information_mode() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "Test content").unwrap();

        let (content, chars, words, lines) =
            process_file_content(&file_path, temp_dir.path(), true).unwrap();

        // In output_information mode, content should be empty
        assert_eq!(content, "");
        assert_eq!(chars, 13); // "Test content\n"
        assert_eq!(words, 2);
        assert_eq!(lines, 1);
    }

    #[test]
    fn test_process_file_content_relative_path() {
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let file_path = subdir.join("nested.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "Nested").unwrap();

        let (content, _chars, _words, _lines) =
            process_file_content(&file_path, temp_dir.path(), false).unwrap();

        assert!(content.contains("**subdir/nested.txt:**"));
    }

    #[test]
    fn test_process_file_content_word_counting() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "one two   three\tfour\nfive").unwrap();

        let (_content, _chars, words, _lines) =
            process_file_content(&file_path, temp_dir.path(), false).unwrap();

        assert_eq!(words, 5);
    }

    #[test]
    fn test_process_file_content_unicode() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "Hello 世界 🌍").unwrap();

        let (_content, chars, words, _lines) =
            process_file_content(&file_path, temp_dir.path(), false).unwrap();

        // Rust's chars().count() counts Unicode scalar values
        // "Hello 世界 🌍\n" = 11 scalar values
        assert_eq!(chars, 11);
        assert_eq!(words, 3);
    }

    #[test]
    fn test_process_file_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.txt");

        let result = process_file_content(&file_path, temp_dir.path(), false);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to open file"));
    }

    #[test]
    fn test_process_file_with_cli() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "CLI test").unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 10,
            include_errors: false,
            output_information: false,
            copy: false,
        };

        let (content, chars, words, lines) = process_file(&cli, &file_path).unwrap();

        assert!(content.contains("**test.txt:**"));
        assert_eq!(chars, 9); // "CLI test\n"
        assert_eq!(words, 2);
        assert_eq!(lines, 1);
    }
}
