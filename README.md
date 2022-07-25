# clipboard-anywhere

[![Crates.io](https://img.shields.io/crates/v/clipboard-anywhere.svg)](https://crates.io/crates/clipboard-anywhere)

A simple wrapper around [`clipboard`](https://lib.rs/crates/clipboard) that works in a few more situations:

- When used in Linux under WSL, it will copy to the Windows clipboard using `clip.exe`
- When used in a remote SSH session, it will use the OSC 52 control sequence to copy to the client clipboard

## Usage

```rust
use clipboard_anywhere::set_clipboard;

let text = "Hello, world!";
clipboard_anywhere::set_clipboard(text)?;
```

## To Do

- [ ] Define custom errors instead of passing everything through Anyhow
- [ ] Write some integration tests
