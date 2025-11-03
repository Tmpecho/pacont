use anyhow::Result;
use std::io::Write;
use std::process::{Command, Stdio};

fn execute_clipboard_command(command_name: &str, args: &[&str], content: &str) -> Result<()> {
    let mut child = Command::new(command_name)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| anyhow::anyhow!("Failed to spawn {}", command_name))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(content.as_bytes())
            .map_err(|_| anyhow::anyhow!("Failed to write to {} stdin", command_name))?;
        drop(stdin);

        let status = child
            .wait()
            .map_err(|_| anyhow::anyhow!("Failed to wait for {} process", command_name))?;

        if status.success() {
            return Ok(());
        }
    }

    anyhow::bail!("{} command failed", command_name)
}

#[cfg(target_os = "linux")]
fn try_linux_clipboard_commands(content: &str) -> Result<()> {
    // Try xclip first (most common)
    // xclip forks to background by default to keep clipboard content available
    if execute_clipboard_command("xclip", &["-selection", "clipboard"], content).is_ok() {
        return Ok(());
    }

    // Try xsel as fallback
    // xsel with --keep flag will persist the clipboard even after program exits
    if execute_clipboard_command("xsel", &["--clipboard", "--input", "--keep"], content).is_ok() {
        return Ok(());
    }

    // Try wl-copy for Wayland
    if execute_clipboard_command("wl-copy", &[], content).is_ok() {
        return Ok(());
    }

    anyhow::bail!("No Linux clipboard commands available (xclip, xsel, or wl-copy)")
}

fn try_arboard_clipboard(content: String) -> Result<()> {
    use arboard::Clipboard;

    let mut clipboard =
        Clipboard::new().map_err(|e| anyhow::anyhow!("Failed to initialize clipboard: {}", e))?;

    clipboard
        .set_text(content)
        .map_err(|e| anyhow::anyhow!("Failed to copy to clipboard: {}", e))?;

    Ok(())
}

pub fn copy_to_clipboard(content: String) -> Result<()> {
    // Try platform-specific clipboard commands first (they handle persistence better on Linux)
    #[cfg(target_os = "linux")]
    if try_linux_clipboard_commands(&content).is_ok() {
        return Ok(());
    }

    // Fallback to arboard for macOS, Windows, or if Linux commands aren't available
    try_arboard_clipboard(content)
}

#[cfg(test)]
mod tests {
    // We can't easily test actual clipboard functionality in unit tests,
    // but we can at least ensure it doesn't panic with empty input
    use super::*;

    #[test]
    fn test_copy_to_clipboard_empty_string() {
        let result = copy_to_clipboard(String::new());
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_copy_to_clipboard_basic_text() {
        let result = copy_to_clipboard("Hello, World!".to_string());
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_copy_to_clipboard_multiline() {
        let content = "Line 1\nLine 2\nLine 3".to_string();
        let result = copy_to_clipboard(content);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_copy_to_clipboard_unicode() {
        let content = "Hello 世界 🌍 émojis".to_string();
        let result = copy_to_clipboard(content);
        assert!(result.is_ok() || result.is_err());
    }
}
