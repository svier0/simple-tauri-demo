
#[tauri::command]
pub fn ipc_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn ipc_server_version() -> String {
    "0.1.0-rc.6".to_string()
}

#[tauri::command]
pub fn ipc_server_latest_ver() -> String {
    simple_tauri::utils::get_npm_latest_ver("@deepseek-ai/dsh").to_string()
}

#[tauri::command]
pub fn ipc_server_update() {
}
