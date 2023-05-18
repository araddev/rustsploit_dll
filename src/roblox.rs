const PRINT_ADDRESS: u32 = 0x107ACC0;

use winapi::{um::libloaderapi, shared::minwindef::DWORD, ctypes::{c_void, c_int, c_char}};
use core::mem::transmute;
use std::i8;

type Rprint = extern fn(c_int, *const c_char) -> usize;

pub unsafe fn x(x: u32) -> *mut c_void {
    (x - 0x400000 + (libloaderapi::GetModuleHandleA(0 as *const i8) as DWORD)) as *mut c_void
}

pub unsafe fn print(printstring: &[u8; 17]){ // i thought this was supposed to be a string and it wasnt and it took me so long to find that out :skull:
    let print: Rprint = transmute(x(PRINT_ADDRESS));
    print(1, transmute(printstring));
}