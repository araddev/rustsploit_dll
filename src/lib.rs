use std::{ptr::null_mut}; 
use jni::{sys::JNI_GetCreatedJavaVMs, JNIEnv, JavaVM};
use winapi::{shared::minwindef::{BOOL, HMODULE, DWORD, LPVOID}, um::{winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}}};
use std::ffi::*;
use winapi::um::*;

struct JVMClassLoader{
env: JNIEnv;
}

fn attach_jvm() -> bool {
    //if (JNI_GetCreatedJavaVMs(, bufLen, nVMs))

return true;
}

unsafe extern "system" fn entry(r:LPVOID) -> DWORD {
    
    0
}

#[no_mangle]
pub unsafe extern "system" fn DllMain( hModule:HMODULE, dw_reason:DWORD, lpReserved:LPVOID ) -> BOOL {
    if dw_reason == DLL_PROCESS_ATTACH {
            
    }
    if dw_reason == DLL_PROCESS_DETACH {
    
    }
    1
}
