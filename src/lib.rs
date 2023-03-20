// the addresses
const PRINT_ADDY: u32 = 0x107ACC0;

// the code that uses the address to print
use std::{ffi::c_void, ptr::null_mut};
use core::mem::transmute;
use winapi::{ctypes::c_char, shared::minwindef::{BOOL, HMODULE, DWORD, LPVOID}, um::{minwinbase::LPTHREAD_START_ROUTINE, winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}, processthreadsapi::ExitProcess}};

unsafe  fn x(x: u32) -> *mut c_void {
    (x - 0x400000 + (winapi::um::libloaderapi::GetModuleHandleA((0 as *const i8)) 
    as winapi::shared::minwindef::DWORD))
     as *mut c_void
}

type r_print_t = extern fn(std::ffi::c_int, *const std::ffi::c_char) -> usize;


extern "system" fn dllentry(r:LPVOID) -> DWORD {
    let r_lua_print: r_print_t = unsafe {transmute(x(PRINT_ADDY)) };
    r_lua_print(1, unsafe {transmute(b"this is a string\0")});
    0
}

#[no_mangle]
pub extern "system" fn DllMain( hModule:HMODULE, dw_reason:DWORD, lpReserved:LPVOID ) -> BOOL {
    if (dw_reason == DLL_PROCESS_ATTACH) {
        unsafe {winapi::um::processthreadsapi::CreateThread(null_mut(), 
            10000000,
            Some(dllentry) , 
            null_mut(),
             0,
              null_mut());}
    }
    if (dw_reason == DLL_PROCESS_DETACH) {

    }
    1
}
