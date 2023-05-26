use std::{ffi::{c_int, c_char}, time::Duration};

pub fn x(address: usize) -> usize {
    let base = unsafe { winapi::um::libloaderapi::GetModuleHandleA(std::ptr::null()) } as usize;
    (address + base) - 0x400000
}

pub fn print(type_val: u8, content: &str) -> c_int {
    let address = x(0xEF9490);
    unsafe {
        let func_ptr: extern "C" fn(u8, *const c_char) -> i32 =
            Some(std::mem::transmute(address)).unwrap();
        let content_ptr = content.as_ptr() as *const c_char;
        func_ptr(type_val, content_ptr)
    }
}

pub fn PrintToRoblox(thingtoprint: &str, mut typeofprint: u8){
    if typeofprint>3 && typeofprint<0 {
        typeofprint = 3;
    }
    print(typeofprint, &format!("{}", thingtoprint));
    std::thread::sleep(Duration::from_secs(1));
}