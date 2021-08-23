use crate::{consts::ConnectedUsbDevices, error::Error};
use udev::Enumerator;

pub(crate) fn enumerate_connected_usb() -> crate::Result<Vec<ConnectedUsbDevices>> {
    let mut output = Vec::new();

    let mut enumerator = match Enumerator::new() {
        Ok(res) => res,
        Err(_) => return Err(Error::Generic("could not get udev enumerator".to_string())),
    };

    for device in enumerator.scan_devices().expect("could not scan devices") {
        if !is_usb_device(&device) {
            continue;
        }
        let _ = || -> Result<(), Box<dyn std::error::Error>> {
            let _id = device
                .property_value("DEVPATH")
                .ok_or(Error::UsbParsingError)?
                .to_str()
                .ok_or(Error::UsbParsingError)?
                .to_string();

            let mut description = device
                .property_value("ID_MODEL_FROM_DATABASE")
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());

            if description.is_none() {
                description = device
                    .property_value("ID_MODEL")
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string());
            }

            let vendor = device
                .property_value("ID_VENDOR")
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());

            let mut serial_number = device
                .property_value("ID_SERIAL_SHORT")
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());

            if serial_number.is_none() {
                serial_number = device
                    .property_value("ID_SERIAL")
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string());
            }

            let volume_label = device
                .property_value("ID_FS_LABEL")
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());

            let filesystem = device
                .property_value("ID_FS_TYPE")
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());

            output.push(ConnectedUsbDevices {
                vendor,
                description,
                serial_number,
                volume_label,
                filesystem,
            });

            Ok(())
        }();
    }

    // remove if filesystem is none, just to stick with only usb drives
    output.retain(|item| !matches!(&item.filesystem.clone().is_none(), true));
    Ok(output)
}

fn is_usb_device(device: &udev::Device) -> bool {
    //! Checks if a device is a USB device
    let device_filename = device.devpath().to_string_lossy().to_string();
    let device_subsystem = device.subsystem().unwrap_or_default();
    device_subsystem.eq_ignore_ascii_case("block") && device_filename.contains("/usb")
}
