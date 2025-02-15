use icsneo_rs::*;
use std::ffi::CString;

trait SerialNumber {
    fn get_serial_number(&self) -> String;
}

impl SerialNumber for neodevice_t {
    fn get_serial_number(&self) -> String {
        let serial: String = self.serial.map(|v| v as u8 as char).into_iter().collect();
        serial
    }
}

fn main() {
    let mut device_count = 255;
    let mut devices = [
        neodevice_t {
            device: 0 as *mut libc::c_void,
            handle: 0i32,
            serial: [0i8; 7],
            type_: 0,
        }; 255];
    unsafe {
        icsneo_findAllDevices(devices.as_mut_ptr(), &mut device_count);
    }
    println!("Found {} device(s)", device_count);

    for i in 0..device_count as usize {
        let device = &devices[i];
        let success;
        let name = unsafe {
            // Open the device
            success = icsneo_openDevice(device);

            // Get the device type string
            let mut name_length : u64 = 0;
            icsneo_getProductNameForType(device.type_, std::ptr::null_mut(), &mut name_length);
            name_length += 1;
            let mut name_buffer = vec![0 as std::os::raw::c_char; name_length as usize];
            icsneo_getProductNameForType(device.type_, name_buffer.as_mut_ptr(), &mut name_length);
            let name = CString::from_vec_with_nul(name_buffer.iter().map(|v| *v as u8).collect()).unwrap();
            let name = name.to_str().unwrap();
            format!("{} {}", name, device.get_serial_number())
        };
        if success {
            println!("Opened {} successfully:\n{:#?}", name, device);
            unsafe {
                let _ = icsneo_closeDevice(device);
            }
        } else {
            println!("Failed to open {:?}!", device);
        }
    }
}