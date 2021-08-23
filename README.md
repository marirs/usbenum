# USBENUM

Usb Enumeration is a cross-platform library that can enumerate USB devices currently connected and get connection history

### Requirements
- Rust 1.52+

### Usage
- Add to depedencies

```toml
[dependencies]
usbenum = "0.1.0"
```

and then

```rust
use usbenum::list_connected_usb_drives;

fn main() {
    match list_connected_usb_drives() {
        Ok(l) => {
            if l.is_empty() {
                println!("No currently connected usb drives")
            } else {
                println!("{:#?}", l)
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
```

- Running the example: `cargo run --example connected_drives`
---
License: MIT