use std::{
    env,
    process::{Command, Stdio},
};

use anyhow::Result;
use arboard::Clipboard;
use duct::cmd;

/// Copy text to the clipboard. Has special handling for WSL and SSH sessions, otherwise
/// falls back to the cross-platform `clipboard` crate
pub fn set_clipboard(text: &str) -> Result<()> {
    if is_wsl::is_wsl() {
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
    if is_wsl::is_wsl() {
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

/// Set the Windows clipboard using powershell.exe in WSL
fn set_wsl_clipboard(s: &str) -> anyhow::Result<()> {
    // In PowerShell, we can escape literal single-quotes
    // in a single-quoted string by doubling them, e.g.
    //
    // 'hello ''world'''
    //
    // gets printed as
    //
    // hello 'world'
    let escaped_s = s.replace("'", "''");

    let mut powershell = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(&format!("Set-Clipboard -Value '{}'", escaped_s))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // Wait until the powershell process is finished before returning
    powershell.wait()?;

    Ok(())
}
