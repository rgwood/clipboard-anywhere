use anyhow::Result;
use clipboard_anywhere;

fn main() -> Result<()> {
    let clipboard_contents = clipboard_anywhere::get_clipboard()?;
    println!("Clipboard contents: {clipboard_contents}");

    let text = "Hello, world!";
    println!("Copying '{text}' to clipboard");
    clipboard_anywhere::set_clipboard(text)?;
    println!("Copy finished successfully");

    Ok(())
}
