// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(windows)]
    {
        let name = windows_sys::core::w!("dsh-single-instance");
        let mutex = unsafe {
            windows_sys::Win32::System::Threading::CreateMutexW(std::ptr::null(), 0, name)
        };
        if mutex.is_null() {
            eprintln!("创建互斥体失败");
        } else {
            let err = unsafe { windows_sys::Win32::Foundation::GetLastError() };
            if err == windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS {
                unsafe {
                    windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW(
                        std::ptr::null_mut(),
                        windows_sys::core::w!("dsh 已在运行"),
                        windows_sys::core::w!("dsh"),
                        0,
                    )
                };
                return;
            }
        }
    }
    dsh_lib::run()
}
