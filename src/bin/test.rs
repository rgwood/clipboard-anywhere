use anyhow::Result;
use clipboard_anywhere;

fn main() -> Result<()> {
    let text = "Hello, world!";
    println!("Copying '{text}' to clipboard");
    clipboard_anywhere::set_clipboard(text)?;
    println!("Copy finished successfully");

    Ok(())
}
