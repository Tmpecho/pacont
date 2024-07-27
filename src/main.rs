mod cli;
mod file_operations;
mod directory_operations;
mod utils;

use anyhow::Result;
use cli::Cli;
use clap::Parser;
use file_operations::process_file;
use directory_operations::process_directory;
use utils::print_output_information;

fn process_path(cli: &Cli, path: &std::path::Path) -> Result<(usize, usize)> {
    if path.is_dir() {
        process_directory(cli, path)
    } else if path.is_file() {
        process_file(cli, path)
    } else {
        if cli.include_errors {
            println!("ERROR: Path '{}' is neither a file nor a directory", path.display());
        }
        Ok((0, 0))
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.paths.is_empty() {
        anyhow::bail!("No paths provided. Use --help for usage information.");
    }

    if cli.output_information {
        print_output_information(&cli)?;
    } else {
        for path in &cli.paths {
            process_path(&cli, path)?;
        }
    }

    Ok(())
}