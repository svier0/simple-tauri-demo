use simple_tauri::simple_tray::ipc_result;

// 导入默认接口
// 不支持通配符pub use xxx::*;
#[allow(unused_imports)]
pub use simple_tauri::ipc_default::{
    ipc_server_version,
    ipc_server_latest_ver,
    ipc_server_update,
    ipc_config,
    ipc_set_config,
    ipc_set_configs,
    ipc_server_status,
    ipc_server_action,
};

/// 获取本体版本号
#[tauri::command]
pub fn ipc_version() -> serde_json::Value {
    let r = env!("CARGO_PKG_VERSION");
    ipc_result!(0,"",r)
}

/// 获取最新版本号
#[tauri::command]
pub fn ipc_latest_ver() -> serde_json::Value {
    let r = simple_tauri::utils::get_latest_ver("github","svier0/simple-tauri-demo");
    ipc_result!(0,"",r)
}

/// 更新本体 TODO:
#[tauri::command]
pub fn ipc_update() -> serde_json::Value {
    ipc_result!(1,"未实现")
}

/// 读取服务日志 TODO:
#[tauri::command]
pub fn ipc_server_logs() -> serde_json::Value {
    ipc_result!(1,"未实现")
}
