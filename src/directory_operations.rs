use crate::cli::Cli;
use crate::file_operations::process_file_content;
use crate::utils::separator;
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

fn handle_file_error(path: &Path, error: &anyhow::Error, include_errors: bool) {
    if include_errors {
        eprintln!("**{}:**", path.display());
        eprintln!("ERROR: Failed to process file: {}", error);
    }
}

fn handle_walk_error(directory: &Path, error: &walkdir::Error, include_errors: bool) {
    if include_errors {
        eprintln!(
            "ERROR: Failed to read entry in {}: {}",
            directory.display(),
            error
        );
    }
}

fn should_add_separator(buffer: &str, content: &str, output_information: bool) -> bool {
    !buffer.is_empty() && !content.is_empty() && !output_information
}

pub fn process_directory(cli: &Cli, directory: &Path) -> Result<(String, usize, usize, usize)> {
    let mut buffer = String::new();
    let mut total_chars = 0;
    let mut total_words = 0;
    let mut total_lines = 0;

    for entry_result in WalkDir::new(directory).max_depth(cli.max_depth) {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                handle_walk_error(directory, &e, cli.include_errors);
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let (content, chars, words, lines) =
            match process_file_content(entry.path(), directory, cli.output_information) {
                Ok(result) => result,
                Err(e) => {
                    handle_file_error(entry.path(), &e, cli.include_errors);
                    continue;
                }
            };

        total_chars += chars;
        total_words += words;
        total_lines += lines;

        if should_add_separator(&buffer, &content, cli.output_information) {
            buffer.push_str(&separator());
            buffer.push('\n');
        }

        buffer.push_str(&content);
    }

    Ok((buffer, total_chars, total_words, total_lines))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_process_directory_basic() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        fs::write(&file1, "Content 1\n").unwrap();
        fs::write(&file2, "Content 2\n").unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 10,
            include_errors: false,
            output_information: false,
            copy: false,
        };

        let (content, chars, words, lines) = process_directory(&cli, temp_dir.path()).unwrap();

        assert!(content.contains("**file1.txt:**"));
        assert!(content.contains("**file2.txt:**"));
        assert!(content.contains("Content 1"));
        assert!(content.contains("Content 2"));
        assert_eq!(chars, 20); // "Content 1\n" + "Content 2\n"
        assert_eq!(words, 4);
        assert_eq!(lines, 2);
    }

    #[test]
    fn test_process_directory_with_subdirs() {
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        fs::write(temp_dir.path().join("root.txt"), "Root\n").unwrap();
        fs::write(subdir.join("nested.txt"), "Nested\n").unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 10,
            include_errors: false,
            output_information: false,
            copy: false,
        };

        let (content, chars, words, lines) = process_directory(&cli, temp_dir.path()).unwrap();

        assert!(content.contains("**root.txt:**"));
        assert!(content.contains("**subdir/nested.txt:**"));
        assert_eq!(chars, 12); // "Root\n" + "Nested\n"
        assert_eq!(words, 2);
        assert_eq!(lines, 2);
    }

    #[test]
    fn test_process_directory_max_depth_zero() {
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        fs::write(temp_dir.path().join("root.txt"), "Root\n").unwrap();
        fs::write(subdir.join("nested.txt"), "Nested\n").unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 0,
            include_errors: false,
            output_information: false,
            copy: false,
        };

        let (content, _chars, _words, _lines) = process_directory(&cli, temp_dir.path()).unwrap();

        // With max_depth 0, we should not traverse into the directory at all
        assert_eq!(content, "");
    }

    #[test]
    fn test_process_directory_max_depth_one() {
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        fs::write(temp_dir.path().join("root.txt"), "Root\n").unwrap();
        fs::write(subdir.join("nested.txt"), "Nested\n").unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 1,
            include_errors: false,
            output_information: false,
            copy: false,
        };

        let (content, _chars, _words, _lines) = process_directory(&cli, temp_dir.path()).unwrap();

        // With max_depth 1, we should see root.txt but not nested.txt
        assert!(content.contains("**root.txt:**"));
        assert!(!content.contains("nested.txt"));
    }

    #[test]
    fn test_process_directory_output_information_mode() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file.txt"), "Test\n").unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 10,
            include_errors: false,
            output_information: true,
            copy: false,
        };

        let (content, chars, words, lines) = process_directory(&cli, temp_dir.path()).unwrap();

        // In output_information mode, content should be empty
        assert_eq!(content, "");
        assert_eq!(chars, 5); // "Test\n"
        assert_eq!(words, 1);
        assert_eq!(lines, 1);
    }

    #[test]
    fn test_process_directory_separators() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file1.txt"), "A\n").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "B\n").unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 10,
            include_errors: false,
            output_information: false,
            copy: false,
        };

        let (content, _chars, _words, _lines) = process_directory(&cli, temp_dir.path()).unwrap();

        // Check that separator is present between files
        assert!(content.contains("--------"));
    }

    #[test]
    fn test_process_directory_empty() {
        let temp_dir = TempDir::new().unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 10,
            include_errors: false,
            output_information: false,
            copy: false,
        };

        let (content, chars, words, lines) = process_directory(&cli, temp_dir.path()).unwrap();

        assert_eq!(content, "");
        assert_eq!(chars, 0);
        assert_eq!(words, 0);
        assert_eq!(lines, 0);
    }
}
