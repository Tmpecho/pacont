mod cli;
mod directory_operations;
mod file_operations;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use utils::{output_information, process_path, separator};
use std::process::{Command, Stdio};
use std::io::Write;

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
        copy_to_clipboard(buffer)?;
        eprintln!("Output copied to clipboard.");
    } else {
        eprintln!("No output to copy to clipboard.");
    }

    Ok(())
}

fn copy_to_clipboard(content: String) -> Result<()> {
    // Try platform-specific clipboard commands first (they handle persistence better on Linux)
    #[cfg(target_os = "linux")]
    {
        // Try xclip first (most common)
        // xclip forks to background by default to keep clipboard content available
        if let Ok(mut child) = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                if stdin.write_all(content.as_bytes()).is_ok() {
                    drop(stdin);
                    // Wait for the parent xclip process (which forks and exits quickly)
                    if let Ok(status) = child.wait() {
                        if status.success() {
                            return Ok(());
                        }
                    }
                }
            }
        }
        
        // Try xsel as fallback
        // xsel with --keep flag will persist the clipboard even after program exits
        if let Ok(mut child) = Command::new("xsel")
            .arg("--clipboard")
            .arg("--input")
            .arg("--keep")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                if stdin.write_all(content.as_bytes()).is_ok() {
                    drop(stdin);
                    if let Ok(status) = child.wait() {
                        if status.success() {
                            return Ok(());
                        }
                    }
                }
            }
        }
        
        // Try wl-copy for Wayland
        if let Ok(mut child) = Command::new("wl-copy")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                if stdin.write_all(content.as_bytes()).is_ok() {
                    drop(stdin);
                    if let Ok(status) = child.wait() {
                        if status.success() {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
    
    // Fallback to arboard for macOS, Windows, or if Linux commands aren't available
    use arboard::Clipboard;
    let mut clipboard = Clipboard::new()
        .map_err(|e| anyhow::anyhow!("Failed to initialize clipboard: {}", e))?;
    clipboard.set_text(content)
        .map_err(|e| anyhow::anyhow!("Failed to copy to clipboard: {}", e))?;
    
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
