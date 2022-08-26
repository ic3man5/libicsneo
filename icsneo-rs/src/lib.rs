// Suppress the flurry of warnings caused by using "C" naming conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    extern crate libc;

    #[test]
    pub fn test_find_device() {
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
    }

    #[test]
    pub fn test_open_close_devices() {
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
            unsafe {
                success = icsneo_openDevice(device);
            }
            if success {
                println!("Opened {:?} successfully!", device);
                unsafe {
                    let _ = icsneo_closeDevice(device);
                }
            } else {
                println!("Failed to open {:?}!", device);
            }
        }
    }
}
