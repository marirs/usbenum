use crate::{consts::ConnectedUsbDevices, error::Error};
use std::{
    ffi::OsStr,
    mem::size_of,
    os::windows::ffi::OsStrExt,
    ptr::{null, null_mut},
};
use winapi::um::setupapi::*;

pub(crate) fn enumerate_connected_usb() -> crate::Result<Vec<ConnectedUsbDevices>> {
    let mut output: Vec<ConnectedUsbDevices> = Vec::new();

    let usb: Vec<u16> = OsStr::new("USB\0").encode_wide().collect();
    let dev_info = unsafe {
        SetupDiGetClassDevsW(
            null(),
            usb.as_ptr(),
            null_mut(),
            DIGCF_ALLCLASSES | DIGCF_PRESENT,
        )
    };

    let mut dev_info_data = SP_DEVINFO_DATA {
        cbSize: size_of::<SP_DEVINFO_DATA>() as u32,
        ..Default::default()
    };

    let mut i = 0;
    while unsafe { SetupDiEnumDeviceInfo(dev_info, i, &mut dev_info_data) } > 0 {
        i += 1;
        let mut buf: Vec<u8> = vec![0; 1000];

        if unsafe {
            SetupDiGetDeviceRegistryPropertyW(
                dev_info,
                &mut dev_info_data,
                SPDRP_HARDWAREID,
                null_mut(),
                buf.as_mut_ptr(),
                buf.len() as u32,
                null_mut(),
            )
        } > 0
        {
            if let Ok((_vendor_id, _product_id)) = extract_vid_pid(buf) {
                buf = vec![0; 1000];
                if unsafe {
                    SetupDiGetDeviceRegistryPropertyW(
                        dev_info,
                        &mut dev_info_data,
                        SPDRP_DEVICEDESC,
                        null_mut(),
                        buf.as_mut_ptr(),
                        buf.len() as u32,
                        null_mut(),
                    )
                } > 0
                {
                    let description = string_from_buf_u8(buf);

                    let mut buf: Vec<u16> = vec![0; 1000];

                    if unsafe {
                        SetupDiGetDeviceInstanceIdW(
                            dev_info,
                            &mut dev_info_data,
                            buf.as_mut_ptr(),
                            buf.len() as u32,
                            null_mut(),
                        )
                    } > 0
                    {
                        let _id = string_from_buf_u16(buf);
                        output.push(ConnectedUsbDevices {
                            vendor: None,
                            description: Some(description),
                            serial_number: None,
                            volume_label: None,
                            filesystem: None
                        });
                    }
                }
            }
        }
    }

    unsafe { SetupDiDestroyDeviceInfoList(dev_info) };

    Ok(output)
}

fn extract_vid_pid(buf: Vec<u8>) -> Result<(u16, u16), Box<dyn std::error::Error + Send + Sync>> {
    let id = string_from_buf_u8(buf).to_uppercase();

    let vid = id.find("VID_").ok_or(Error::UsbParsingError)?;
    let pid = id.find("PID_").ok_or(Error::UsbParsingError)?;

    Ok((
        u16::from_str_radix(&id[vid + 4..vid + 8], 16)?,
        u16::from_str_radix(&id[pid + 4..pid + 8], 16)?,
    ))
}

fn string_from_buf_u16(buf: Vec<u16>) -> String {
    let mut out = String::from_utf16_lossy(&buf);

    if let Some(i) = out.find('\u{0}') {
        out.truncate(i);
    }

    out
}

fn string_from_buf_u8(buf: Vec<u8>) -> String {
    let str_vec: Vec<u16> = buf
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
        .collect();

    string_from_buf_u16(str_vec)
}
