use simple_tauri::simple_tray;
use simple_tauri::simple_serve;
use simple_tauri::utils::sh2rs;

mod ipc;

#[cfg(windows)]
pub fn run() {
    // 互斥 只能启动一个实例
    simple_tray::mutex!();
    // 设置ipc函数 自动扫描设定的模块
    simple_tray::set_ipc_cmds![ipc];
    // 窗口列表 [id 标题 url 宽 高 有边框]
    simple_tray::set_window_list!(r#"[
            ["main", "", "http://127.0.0.1:3080", 1280.0, 800.0],
            ["setting", "设置","",null,null,false],
            ["load", "Loading","",380,280,false],
        ]"#);
    // 托盘菜单
    simple_tray::set_tray_menu!(r#"[
            ["show", "显示主界面"],
            ["toggle"],
            ["show-setting", "设置", "show_setting"],
            [],
            ["light"],
        ]"#);
    simple_tray::hooks!(on_tray_before, _, on_quit);
    simple_tray::run!();
}

// 托盘创建前回调
fn on_tray_before() -> Result<(), String> {
    // 设置参数
    let workdir = "server/dsh-<ver>";
    let autoupdate = false;

    //
    simple_serve::set_pkg("npm","@deepseek-ai/dsh");
    simple_serve::set_start_cmd!("run.bat");//DSH_HOME=%USERPROFILE%\.dsh ;../node node_modules/@deepseek-ai/dsh/lib/bin.js web
    simple_serve::set_download_url(
        |ver|format!("https://registry.npmjs.org/@deepseek-ai/dsh/-/dsh-{}.tgz",ver),
        "package");
    // simple_serve::set_ensure_server(|ver,dir|{
    //     let url = format!("https://registry.npmjs.org/@deepseek-ai/dsh/-/dsh-{}.tgz"
    //        ,ver);
    //     let extract_dir = "package";
    //     simple_tauri::utils::unzip_remote(&url,dir,extract_dir)?;
    //     Ok(())
    // });
    simple_serve::set_work_dir(&workdir);
    if autoupdate { simple_serve::enable_auto_update(); }

    // 显示加载窗口
    simple_tray::show_window("load");
    sh2rs!("sleep 1").ok();
    let show_load_tips = |s: &str|{
        simple_tray::runjs("load", &format!("document.querySelector('p.tips').innerHTML='{}'",s));
    };
    show_load_tips("检测本地服务版本");

    // 检查版本更新
    let _ = simple_serve::check_update(false)?;
    // 启动服务
    show_load_tips("服务启动中");
    simple_serve::start()
        .map_err(|e| format!("服务器启动失败: {e}"))?;
    // 检测端口3080 拉起成功才返回继续走托盘创建逻辑
    simple_serve::wait_port(3080).map_err(|e| format!("服务器启动超时: {e}"))?;
    // 关闭加载窗口
    simple_tray::close_window("load");
    // 显示主窗口
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
