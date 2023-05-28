use std::ffi::c_void;
mod roblox;

unsafe fn main_thread() {
        roblox::print_to_roblox("this is a string", 0);
        roblox::print_to_roblox("this is a cooler string", 1);
        roblox::print_to_roblox("this is a very cool string", 2);
}


#[no_mangle]
pub unsafe extern "system" fn DllMain( mod_handle: winapi::shared::minwindef::HINSTANCE, reason: u32, _: *mut c_void, ) -> i32 {
    winapi::um::libloaderapi::DisableThreadLibraryCalls(mod_handle);
    if reason == winapi::um::winnt::DLL_PROCESS_ATTACH {
        main_thread();
    }
    if reason == winapi::um::winnt::DLL_PROCESS_DETACH{
        println!("wtf the dll detached");
    }
    1
}
