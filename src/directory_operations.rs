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
