# clipboard-anywhere

[![Crates.io](https://img.shields.io/crates/v/clipboard-anywhere.svg)](https://crates.io/crates/clipboard-anywhere)

A simple wrapper around [`arboard`](https://github.com/1Password/arboard) that works in a few more situations:

- In Linux under WSL, it can copy to and from the Windows clipboard (using `clip.exe` and `powershell get-clipboard`)
- In a remote SSH session, can copy to the local clipboard using the OSC 52 control sequence

## Usage

```rust
// Attempt to get clipboard contents. Will return error in an SSH session
let clipboard_contents: String = clipboard_anywhere::get_clipboard()?;

// Set clipboard contents to "Hello, world!";
clipboard_anywhere::set_clipboard("Hello, world!")?;
```

## To Do

- [ ] Define custom errors instead of passing everything through Anyhow
- [ ] Write some integration tests
