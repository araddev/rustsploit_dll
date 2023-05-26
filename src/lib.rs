use std::ffi::c_void;
mod roblox;



fn main_thread() {
        roblox::PrintToRoblox("this is a string", 0);
        roblox::PrintToRoblox("this is a cooler string", 1);
        roblox::PrintToRoblox("this is a very cool string", 2);
}

#[no_mangle]
pub unsafe extern "system" fn DllMain( mod_handle: winapi::shared::minwindef::HINSTANCE, reason: u32, _: *mut c_void, ) -> i32 {
    winapi::um::libloaderapi::DisableThreadLibraryCalls(mod_handle);
    if reason == winapi::um::winnt::DLL_PROCESS_ATTACH {
        main_thread();
    }
    1
}
