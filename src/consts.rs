use serde::{Serialize, Deserialize};

#[derive(PartialEq, Hash, Clone, Default, Deserialize, Serialize)]
pub struct ConnectedUsbDevices {
    pub vendor: Option<String>,
    pub description: Option<String>,
    pub serial_number: Option<String>,
    pub volume_label: Option<String>,
    pub filesystem: Option<String>,
}

pub(crate) fn is_usb_device(device: &udev::Device) -> bool {
    //! Checks if a device is a USB device
    let device_filename = device.devpath().to_string_lossy().to_string();
    let device_subsystem = device.subsystem().unwrap_or_default();
    if device_subsystem.eq_ignore_ascii_case("block") && device_filename.contains("/usb") {
        true
    } else {
        false
    }
}