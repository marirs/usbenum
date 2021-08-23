# USBENUM
![Crates.io](https://img.shields.io/crates/v/usbenum)
![GitHub](https://img.shields.io/github/license/marirs/usbenum)
[![Windows](https://github.com/marirs/usbenum/actions/workflows/windows.yml/badge.svg)](https://github.com/marirs/usbenum/actions/workflows/windows.yml)
[![MacOs](https://github.com/marirs/usbenum/actions/workflows/macos.yml/badge.svg)](https://github.com/marirs/usbenum/actions/workflows/macos.yml)
[![Ubuntu](https://github.com/marirs/usbenum/actions/workflows/linux.yml/badge.svg)](https://github.com/marirs/usbenum/actions/workflows/linux.yml)
[![Armv7 Linux](https://github.com/marirs/usbenum/actions/workflows/linux_arm.yml/badge.svg)](https://github.com/marirs/usbenum/actions/workflows/linux_arm.yml)

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