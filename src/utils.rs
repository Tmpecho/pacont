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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_separator() {
        let sep = separator();
        assert_eq!(sep.len(), 80);
        assert!(sep.chars().all(|c| c == '-'));
    }

    #[test]
    fn test_process_path_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello World\n").unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 10,
            include_errors: false,
            output_information: false,
            copy: false,
        };

        let (content, chars, words, lines) = process_path(&cli, &file_path).unwrap();

        assert!(content.contains("**test.txt:**"));
        assert!(content.contains("Hello World"));
        assert_eq!(chars, 12);
        assert_eq!(words, 2);
        assert_eq!(lines, 1);
    }

    #[test]
    fn test_process_path_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file.txt"), "Content\n").unwrap();

        let cli = Cli {
            paths: vec![],
            max_depth: 10,
            include_errors: false,
            output_information: false,
            copy: false,
        };

        let (content, chars, words, lines) = process_path(&cli, temp_dir.path()).unwrap();

        assert!(content.contains("**file.txt:**"));
        assert_eq!(chars, 8);
        assert_eq!(words, 1);
        assert_eq!(lines, 1);
    }

    #[test]
    fn test_output_information_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello\nWorld\n").unwrap();

        let cli = Cli {
            paths: vec![file_path.clone()],
            max_depth: 10,
            include_errors: false,
            output_information: true,
            copy: false,
        };

        let info = output_information(&cli).unwrap();

        assert!(info.contains("Paths:"));
        assert!(info.contains("test.txt"));
        assert!(info.contains("Total Characters: 12"));
        assert!(info.contains("Total Words: 2"));
        assert!(info.contains("Total Non-Empty Lines: 2"));
    }

    #[test]
    fn test_output_information_multiple_paths() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "One\n").unwrap();
        fs::write(&file2, "Two Three\n").unwrap();

        let cli = Cli {
            paths: vec![file1, file2],
            max_depth: 10,
            include_errors: false,
            output_information: true,
            copy: false,
        };

        let info = output_information(&cli).unwrap();

        assert!(info.contains("Total Characters: 14")); // "One\n" + "Two Three\n"
        assert!(info.contains("Total Words: 3"));
        assert!(info.contains("Total Non-Empty Lines: 2"));
    }

    #[test]
    fn test_output_information_directory() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file1.txt"), "A B\n").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "C D E\n").unwrap();

        let cli = Cli {
            paths: vec![temp_dir.path().to_path_buf()],
            max_depth: 10,
            include_errors: false,
            output_information: true,
            copy: false,
        };

        let info = output_information(&cli).unwrap();

        assert!(info.contains("Total Characters: 10")); // "A B\n" + "C D E\n"
        assert!(info.contains("Total Words: 5"));
        assert!(info.contains("Total Non-Empty Lines: 2"));
    }

    #[test]
    fn test_output_information_empty_lines() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Line1\n\n\nLine2\n").unwrap();

        let cli = Cli {
            paths: vec![file_path],
            max_depth: 10,
            include_errors: false,
            output_information: true,
            copy: false,
        };

        let info = output_information(&cli).unwrap();

        assert!(info.contains("Total Non-Empty Lines: 2")); // Only Line1 and Line2
    }
}
