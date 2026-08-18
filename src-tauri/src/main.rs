// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(windows)]
    if simple_tauri_lib::check_mutex("dsh") {
        return;
    }
    dsh_lib::run()
}
