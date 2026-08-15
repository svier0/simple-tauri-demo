use simple_tauri_lib::simple_tray;
use simple_tauri_lib::simple_serve;

#[tauri::command]
fn ipc_version() -> String {
    "1.0.0".to_string()
}

#[cfg(windows)]
pub fn run() {
    simple_tray::set_ipc_cmds![
            ipc_version,
        ];
    simple_tray::set_window_list!(r#"[
            ["main", "", "http://127.0.0.1:3080", 1280.0, 800.0],
            ["setting", "设置","",null,null,false],
            ["load", "Loading","",380,280,false],
        ]"#);
    simple_tray::set_tray_menu!(r#"[
            ["show", "显示主界面"],
            ["show-setting", "设置", "show_setting"],
            [],
            ["light"],
        ]"#);
    simple_tray::hooks!(on_tray_before, on_tray_after, on_quit);
    simple_tray::run!();
}

// 托盘创建前回调
fn on_tray_before() -> Result<(), String> {
    // 显示加载窗口
    simple_tray::show_window("load");
    // 启动服务器
    simple_serve::start("server/dsh-0.1.0", "run.bat").map_err(|e| format!("服务器启动失败: {e}"))?;
    // 检测端口3080 拉起成功才返回继续走托盘创建逻辑
    simple_serve::wait_port(3080).map_err(|e| format!("服务器启动超时: {e}"))?;
    // 关闭加载窗口
    simple_tray::close_window("load");
    Ok(())
}

// 托盘创建后回调
fn on_tray_after() -> Result<(), String> {
    simple_tray::show_window("main");
    Ok(())
}

// 点击"退出"时的回调
fn on_quit() -> Result<(), String> {
    simple_serve::stop();
    Ok(())
}

fn show_setting() {
    // simple_tray::show_window("setting");
    simple_tray::show_window("main");
    simple_tray::runjs("main", r#"document.querySelector('[data-slot="sidebar.settings"]>button').click()"#);
}