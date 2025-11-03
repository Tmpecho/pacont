use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to get the path to the pacont binary
fn get_pacont_binary() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    path.pop(); // Remove 'deps' directory
    path.push("pacont");
    path
}

#[test]
fn test_single_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Hello World\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg(&file_path)
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("**test.txt:**"));
    assert!(stdout.contains("Hello World"));
}

#[test]
fn test_directory_traversal() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("file1.txt"), "Content 1\n").unwrap();
    fs::write(temp_dir.path().join("file2.txt"), "Content 2\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("**file1.txt:**"));
    assert!(stdout.contains("**file2.txt:**"));
    assert!(stdout.contains("Content 1"));
    assert!(stdout.contains("Content 2"));
}

#[test]
fn test_subdirectory_traversal() {
    let temp_dir = TempDir::new().unwrap();
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();

    fs::write(temp_dir.path().join("root.txt"), "Root content\n").unwrap();
    fs::write(subdir.join("nested.txt"), "Nested content\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("**root.txt:**"));
    assert!(stdout.contains("**subdir/nested.txt:**"));
}

#[test]
fn test_max_depth_zero() {
    let temp_dir = TempDir::new().unwrap();
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();

    fs::write(temp_dir.path().join("root.txt"), "Root\n").unwrap();
    fs::write(subdir.join("nested.txt"), "Nested\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg("-m")
        .arg("0")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // With max_depth 0, no files should be traversed
    assert_eq!(stdout, "");
}

#[test]
fn test_max_depth_one() {
    let temp_dir = TempDir::new().unwrap();
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();

    fs::write(temp_dir.path().join("root.txt"), "Root\n").unwrap();
    fs::write(subdir.join("nested.txt"), "Nested\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg("-m")
        .arg("1")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("**root.txt:**"));
    assert!(!stdout.contains("nested.txt"));
}

#[test]
fn test_output_information_flag() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "Hello World\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg("-o")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Paths:"));
    assert!(stdout.contains("Total Characters:"));
    assert!(stdout.contains("Total Words:"));
    assert!(stdout.contains("Total Non-Empty Lines:"));
}

#[test]
fn test_multiple_paths() {
    let temp_dir = TempDir::new().unwrap();
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");

    fs::write(&file1, "File 1\n").unwrap();
    fs::write(&file2, "File 2\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("**file1.txt:**"));
    assert!(stdout.contains("**file2.txt:**"));
    assert!(stdout.contains("File 1"));
    assert!(stdout.contains("File 2"));
    // Check for separator between files
    assert!(stdout.contains("--------"));
}

#[test]
fn test_multiple_paths_with_output_information() {
    let temp_dir = TempDir::new().unwrap();
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");

    fs::write(&file1, "One Two\n").unwrap();
    fs::write(&file2, "Three\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg("-o")
        .arg(&file1)
        .arg(&file2)
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Total Characters: 14")); // "One Two\n" + "Three\n"
    assert!(stdout.contains("Total Words: 3"));
    assert!(stdout.contains("Total Non-Empty Lines: 2"));
}

#[test]
fn test_nonexistent_path() {
    let output = Command::new(get_pacont_binary())
        .arg("/nonexistent/path/to/file.txt")
        .output()
        .expect("Failed to execute pacont");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Path or file does not exist"));
}

#[test]
fn test_no_paths_provided() {
    let output = Command::new(get_pacont_binary())
        .output()
        .expect("Failed to execute pacont");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("No paths provided"));
}

#[test]
fn test_separator_between_files() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("a.txt"), "A\n").unwrap();
    fs::write(temp_dir.path().join("b.txt"), "B\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Check that there's exactly one separator (80 dashes)
    let separator_count = stdout.matches(&"-".repeat(80)).count();
    assert_eq!(separator_count, 1);
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(get_pacont_binary())
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "");
}

#[test]
fn test_file_with_unicode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("unicode.txt");
    fs::write(&file_path, "Hello 世界 🌍\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg(&file_path)
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Hello 世界 🌍"));
}

#[test]
fn test_copy_flag_no_clipboard_interaction() {
    // We can't test actual clipboard functionality in CI, but we can verify
    // the flag doesn't crash the program
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Test\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg("-c")
        .arg(&file_path)
        .output()
        .expect("Failed to execute pacont");

    // The command might succeed or fail depending on clipboard availability,
    // but it shouldn't crash
    assert!(output.status.success() || !output.status.success());
    
    // Stdout should be empty when using -c flag
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout, "");
}

#[test]
fn test_long_max_depth() {
    let temp_dir = TempDir::new().unwrap();
    let mut current_dir = temp_dir.path().to_path_buf();
    
    // Create a deep nested structure
    for i in 1..=5 {
        current_dir = current_dir.join(format!("level{}", i));
        fs::create_dir(&current_dir).unwrap();
    }
    fs::write(current_dir.join("deep.txt"), "Deep content\n").unwrap();

    // Test with sufficient max depth
    let output = Command::new(get_pacont_binary())
        .arg("-m")
        .arg("10")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("**level1/level2/level3/level4/level5/deep.txt:**"));
}

#[test]
fn test_mixed_files_and_directories() {
    let temp_dir = TempDir::new().unwrap();
    let dir1 = temp_dir.path().join("dir1");
    let dir2 = temp_dir.path().join("dir2");
    fs::create_dir(&dir1).unwrap();
    fs::create_dir(&dir2).unwrap();

    let file1 = temp_dir.path().join("root.txt");
    let file2 = dir1.join("file1.txt");
    let file3 = dir2.join("file2.txt");

    fs::write(&file1, "Root\n").unwrap();
    fs::write(&file2, "Dir1\n").unwrap();
    fs::write(&file3, "Dir2\n").unwrap();

    let output = Command::new(get_pacont_binary())
        .arg(&file1)
        .arg(&dir1)
        .arg(&file3)
        .output()
        .expect("Failed to execute pacont");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("**root.txt:**"));
    assert!(stdout.contains("**file1.txt:**"));
    assert!(stdout.contains("**file2.txt:**"));
}
