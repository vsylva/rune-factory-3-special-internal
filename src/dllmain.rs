#[unsafe(no_mangle)]
unsafe extern "system" fn DllMain(
    h_module: isize,
    ul_reason_for_call: u32,
    _lp_reserved: *mut ::core::ffi::c_void,
) -> i32 {
    if ul_reason_for_call == 1 {
        hudhook::windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls(
            hudhook::windows::Win32::Foundation::HMODULE(h_module),
        )
        .unwrap();

        ::std::thread::spawn(move || crate::init::on_game_start(h_module));
    }

    1
}
