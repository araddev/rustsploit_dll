use std::{ptr::null_mut}; 
use winapi::{shared::minwindef::{BOOL, HMODULE, DWORD, LPVOID}, um::{winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}}};
use std::ffi::*;
use winapi::um::*;

mod roblox;

type Rprint = extern fn(c_int, *const c_char) -> usize;

unsafe extern "system" fn entry(r:LPVOID) -> DWORD {
    roblox::print(b"this is a string\0");
    0
}

#[no_mangle]
pub unsafe extern "system" fn DllMain( hModule:HMODULE, dw_reason:DWORD, lpReserved:LPVOID ) -> BOOL {
    if dw_reason == DLL_PROCESS_ATTACH {
            processthreadsapi::CreateThread(
                null_mut(), 
                10000000,
                Some(entry), 
                null_mut(),
                0,
                null_mut()
            );
    }
    if dw_reason == DLL_PROCESS_DETACH {
               print!("what the fuck the dll detached");
    }
    1
}
