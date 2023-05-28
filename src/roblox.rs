use std::{ffi::{c_int, c_char}, time::Duration};
use std::ffi::CString;

pub fn x(address: usize) -> usize {
    let base = unsafe { winapi::um::libloaderapi::GetModuleHandleA(std::ptr::null()) } as usize;
    (address + base) - 0x400000
}

pub unsafe fn print(type_val: u8, content: CString) -> c_int {
    let address = x(0xEF9490);
        let func_ptr: extern "C" fn(u8, *const c_char) -> i32 = Some(std::mem::transmute(address)).unwrap();
        let content_ptr = content.as_ptr() as *const c_char;
        func_ptr(type_val, content_ptr)
}

pub unsafe fn print_to_roblox(thingtoprint: &str, mut typeofprint: u8){
    if typeofprint>3 { typeofprint = 3; }
    print(typeofprint, convert_to_cstring(thingtoprint));
    std::thread::sleep(Duration::from_secs(1));
}

fn convert_to_cstring(input: &str) -> CString {
    match CString::new(input) {
        Ok(cstring) => cstring,
        Err(_) => panic!("Failed to convert &str to CString"),
    }
}