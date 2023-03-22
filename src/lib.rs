const PRINT_ADDRESS: u32 = 0x107ACC0;

use std::{ffi::c_void, ptr::null_mut, fmt::Write};
use core::mem::transmute;
use winapi::{shared::minwindef::{BOOL, HMODULE, DWORD, LPVOID}, um::{winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}}};
use std::ffi::*;
use winapi::um::*;

unsafe fn x(x: u32) -> *mut c_void {
    (x - 0x400000 + (libloaderapi::GetModuleHandleA(0 as *const i8) as DWORD)) as *mut c_void
}

type Rprint = extern fn(c_int, *const c_char) -> usize;

extern "system" fn entry(r:LPVOID) -> DWORD {
    let print: Rprint = unsafe {transmute(x(PRINT_ADDRESS)) };
    
    let mut address_str = String::new();
    write!(&mut address_str, "Address: 0x{:X}\0", PRINT_ADDRESS).unwrap();
    print(1, address_str.as_ptr() as *const c_char);
    
    print(1, unsafe {transmute(b"this is a string\0")});
    print(1, unsafe {transmute(b"printsploit winning\0")});
    0
}

#[no_mangle]
pub extern "system" fn DllMain( hModule:HMODULE, dw_reason:DWORD, lpReserved:LPVOID ) -> BOOL {
    if (dw_reason == DLL_PROCESS_ATTACH) {
        unsafe {
            processthreadsapi::CreateThread(
                null_mut(),
                10000000,
                Some(entry),
                null_mut(),
                0,
                null_mut()
            );
        }
    }

    if (dw_reason == DLL_PROCESS_DETACH) {
        //will probs have to put shit here later
    }
    1
}
