use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;
use arboard::Clipboard;
use duct::cmd;

/// Copy text to the clipboard. Has special handling for WSL and SSH sessions, otherwise
/// falls back to the cross-platform `clipboard` crate
pub fn set_clipboard(text: &str) -> Result<()> {
    if wsl::is_wsl() {
        set_wsl_clipboard(text)?;
    } else if env::var("SSH_CLIENT").is_ok() {
        // we're in an SSH session, so set the clipboard using OSC 52 escape sequence
        set_clipboard_osc_52(text);
    } else {
        // we're probably running on a host/primary OS, so use the default clipboard
        match Clipboard::new() {
            Ok(mut ctx) => {
                if let Err(e) = ctx.set_text(text.to_string()) {
                    anyhow::bail!("Failed to set clipboard: {e}");
                }
            }
            Err(e) => anyhow::bail!("Failed to create clipboard context: {e}"),
        }
    }

    Ok(())
}

pub fn get_clipboard() -> Result<String> {
    if wsl::is_wsl() {
        let stdout = cmd!("powershell.exe", "get-clipboard").read()?;
        Ok(stdout.trim().to_string())
    } else if env::var("SSH_CLIENT").is_ok() {
        anyhow::bail!("SSH clipboard not supported");
    } else {
        // we're probably running on a host/primary OS, so use the default clipboard
        match Clipboard::new() {
            Ok(mut ctx) => match ctx.get_text() {
                Ok(text) => Ok(text),
                Err(e) => anyhow::bail!("Failed to get clipboard: {e}"),
            },
            Err(e) => anyhow::bail!("Failed to create clipboard context: {e}"),
        }
    }
}

/// Set the clipboard contents using OSC 52 (picked up by most terminals)
fn set_clipboard_osc_52(text: &str) {
    print!("\x1B]52;c;{}\x07", base64::encode(text));
}

/// Set the Windows clipboard using clip.exe in WSL
fn set_wsl_clipboard(s: &str) -> anyhow::Result<()> {
    let mut clipboard = Command::new("clip.exe").stdin(Stdio::piped()).spawn()?;
    let mut clipboard_stdin = clipboard
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("Could not get stdin handle for clip.exe"))?;

    clipboard_stdin.write_all(s.as_bytes())?;

    Ok(())
}
