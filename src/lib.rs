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
