use anyhow::Result;
use crate::cli::Cli;
use crate::process_path;

pub fn separator() {
    println!("{}", "-".repeat(80));
}

pub fn print_output_information(cli: &Cli) -> Result<()> {
    let mut total_chars = 0;
    let mut total_words = 0;
    let mut paths = String::new();

    for path in &cli.paths {
        let (char_count, word_count) = process_path(cli, path)?;
        total_chars += char_count;
        total_words += word_count;
        if !paths.is_empty() {
            paths.push(' ');
        }
        paths.push_str(&path.display().to_string());
    }

    println!("Paths: {}", paths);
    println!("Total Characters: {}", total_chars);
    println!("Total Words: {}", total_words);

    Ok(())
}