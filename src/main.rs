mod cli;
mod directory_operations;
mod file_operations;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use utils::{output_information, process_path, separator};

use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.paths.is_empty() {
        anyhow::bail!("No paths provided. Use --help for usage information.");
    }

    for path in &cli.paths {
        if !path.exists() {
            anyhow::bail!("Path or file does not exist: {}", path.display());
        }
    }

    let buffer: String = if cli.output_information {
        output_information(&cli)?
    } else {
        output_content(&cli)?
    };

    if !cli.copy {
        print!("{}", buffer);
    } else if !buffer.is_empty() {
        ClipboardContext::new()
            .map_err(|e| anyhow::anyhow!("Failed to initialize clipboard: {}", e))?
            .set_contents(buffer)
            .map_err(|e| anyhow::anyhow!("Failed to copy to clipboard: {}", e))?;
        eprintln!("Output copied to clipboard.");
    } else if !cli.output_information {
        eprintln!("Nothing to copy: No content generated or an error occurred.");
    }

    Ok(())
}

fn output_content(cli: &Cli) -> Result<String> {
    let mut buf = String::new();

    for (i, path) in cli.paths.iter().enumerate() {
        match process_path(cli, path) {
            Ok((content, _chars, _words, _lines)) => {
                if i > 0 && !content.is_empty() && !buf.is_empty() {
                    buf.push_str(&separator());
                    buf.push('\n');
                }
                buf.push_str(&content);
            }
            Err(e) => {
                if cli.include_errors {
                    eprintln!("ERROR processing path {}: {}", path.display(), e);
                }
            }
        }
    }
    Ok(buf)
}
