mod consts;
pub use consts::ConnectedUsbDevices;

pub mod error;
pub type Result<T> = std::result::Result<T, error::Error>;

#[cfg(windows)]
#[path = "platforms/windows.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "platforms/linux.rs"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "platforms/macos.rs"]
mod platform;

pub fn list_connected_usb_drives() -> Result<Vec<ConnectedUsbDevices>> {
    //! Get a list of currently connected USB drives
    //! ## Example
    //!
    //! ```rust
    //! use usbenum::list_connected_usb_drives;
    //!
    //! match list_connected_usb_drives() {
    //!     Ok(l) => println!("{:?}", l),
    //!     Err(e) => println!("Error: {:?}", e),
    //! }
    //! ```
    platform::enumerate_connected_usb()
}
