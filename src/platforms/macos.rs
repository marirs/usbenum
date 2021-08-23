use crate::{consts::ConnectedUsbDevices, error::Error};
use core_foundation::{base::*, dictionary::*, number::*, string::*};
use io_kit_sys::{types::*, usb::lib::*, *};
use mach::kern_return::*;
use std::mem::MaybeUninit;

/// Enumerate connected usb drives
pub(crate) fn enumerate_connected_usb() -> crate::Result<Vec<ConnectedUsbDevices>> {
    let mut output = Vec::new();

    unsafe {
        let matching_dict = IOServiceMatching(kIOUSBDeviceClassName);
        if matching_dict.as_ref().is_none() {
            return Err(Error::FailedIoService);
        }

        let mut iter: io_iterator_t = 0;

        let kr = IOServiceGetMatchingServices(kIOMasterPortDefault, matching_dict, &mut iter);
        if kr != KERN_SUCCESS {
            return Err(Error::FailedIoService);
        }

        #[allow(unused_assignments)]
        let mut device: io_service_t = 0;

        #[allow(clippy::unit_cmp)]
        while (device = IOIteratorNext(iter)) == () && device > 0 {
            #[allow(clippy::uninit_assumed_init)]
            let mut props: CFMutableDictionaryRef = MaybeUninit::uninit().assume_init();

            let _result =
                IORegistryEntryCreateCFProperties(device, &mut props, kCFAllocatorDefault, 0);

            let properties: CFDictionary<CFString, CFType> =
                CFMutableDictionary::wrap_under_get_rule(props).to_immutable();

            let _ = || -> Result<(), Box<dyn std::error::Error>> {
                let key = CFString::from_static_string("sessionID");
                let _id = properties
                    .find(&key)
                    .and_then(|value_ref| value_ref.downcast::<CFNumber>())
                    .ok_or(Error::UsbParsingError)?
                    .to_i64()
                    .ok_or(Error::UsbParsingError)?;

                let key = CFString::from_static_string("USB Serial Number");
                let serial_number = properties
                    .find(&key)
                    .and_then(|value_ref| value_ref.downcast::<CFString>())
                    .map(|s| s.to_string());

                let key = CFString::from_static_string("USB Vendor Name");
                let vendor = properties
                    .find(&key)
                    .and_then(|value_ref| value_ref.downcast::<CFString>())
                    .map(|s| s.to_string());

                let key = CFString::from_static_string("USB Product Name");
                let description = properties
                    .find(&key)
                    .and_then(|value_ref| value_ref.downcast::<CFString>())
                    .map(|s| s.to_string());

                output.push(ConnectedUsbDevices {
                    vendor,
                    description,
                    serial_number,
                    volume_label: None,
                    filesystem: None,
                });

                Ok(())
            }();

            IOObjectRelease(device);
        }

        IOObjectRelease(iter);
    }

    Ok(output)
}
