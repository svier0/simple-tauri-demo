use simple_tauri::simple_tray;
use simple_tauri::simple_serve;
use simple_tauri::utils::sh2rs::sh2rs;
use simple_tauri::utils::sh2rs::try_quote;
use indoc::indoc;

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

fn show_load_tips(s: &str){
    simple_tray::runjs("load", &format!("document.querySelector('p.tips').innerHTML='{}'",s));
}

#[cfg(windows)]
fn run_script_install(port: i32,server_home: &str) -> Result<(),String> {
    sh2rs!("echo \"{{}}\" > package.json").ok();
    let cmd = simple_tauri::utils::get_node_cmd(&format!(indoc! {r#"
            call pnpm install @deepseek-ai/dsh
            call pnpm approve-builds -all
        "#}),"");
    sh2rs!("sh {}",try_quote!("{}",cmd))?;
    let cmd = simple_tauri::utils::get_node_cmd(&format!(indoc! {r#"
            set "DSH_HOME={}"
            set "PATH=%DSH_HOME%/node_modules/.bin;%PATH%"
            dsh web --no-open --port {} --trusted-host 127.0.0.1
        "#},server_home,port),"");
    sh2rs!("echo {} > start.cmd",try_quote!("{}", cmd)).ok();
    // 启动服务时执行的命令
    simple_serve::set_start_cmd!("start.bat");
    Ok(())
}

#[cfg(not(windows))]
fn run_script_install(port: i32,server_home: &str) -> Result<(),String> {
    sh2rs!("echo \"{{}}\" > package.json").ok();
    let cmd = simple_tauri::utils::get_node_cmd(&format!(indoc! {r#"
            pnpm install @deepseek-ai/dsh
            pnpm approve-builds -all
        "#}),"");
    sh2rs!("sh {}",try_quote!("{}",cmd))?;
    let cmd = simple_tauri::utils::get_node_cmd(&format!(indoc! {r#"
            export DSH_HOME="{}"
            export PATH="$DSH_HOME/node_modules/.bin:$PATH"
            dsh web --no-open --port {} --trusted-host 127.0.0.1
        "#},server_home,port),"");
    sh2rs!("echo {} > start.sh",try_quote!("{}", cmd)).ok();
    sh2rs!("chmod 755 ./start.sh").ok();
    // 启动服务时执行的命令
    simple_serve::set_start_cmd!("start.sh");
    Ok(())
}

// 托盘创建前回调
fn on_tray_before() -> Result<(), String> {
    // 显示加载窗口
    simple_tray::show_window("load");
    sh2rs!("sleep 1").ok();
    // 设置参数
    let autoupdate = false;
    let port = 34333;
    let server_home = "";

    // 包类型/包名 用于检测服务版本号
    simple_serve::set_pkg("npm","@deepseek-ai/dsh");
    // 需要更新服务端时的回调
    simple_serve::set_ensure_server(move |_ver,dir|{
        sh2rs!("mkdir -p {}",dir).ok();
        sh2rs!("cd {}",dir).ok();
        show_load_tips("安装node");
        simple_tauri::utils::ensure_node("","")?;
        show_load_tips("安装pnpm");
        simple_tauri::utils::ensure_pnpm("","")?;
        show_load_tips("安装dsh");
        run_script_install(port,server_home)?;
        Ok(())
    });
    if autoupdate { simple_serve::enable_auto_update(); }

    // 检查版本更新
    show_load_tips("检测本地服务版本");
    let _ = simple_serve::check_update(false)?;
    // 启动服务
    show_load_tips("服务启动中");
    simple_serve::start()
        .map_err(|e| format!("服务器启动失败: {e}"))?;
    show_load_tips("服务启动中 50%");
    sh2rs!("sleep 1").ok();
    // 检测端口拉起成功才返回继续走托盘创建逻辑
    simple_serve::wait_port(port)
        .map_err(|e| format!("服务器启动超时: {e}"))?;
    show_load_tips("服务启动中 100%");
    sh2rs!("sleep 1").ok();

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
    simple_tray::show_window("main");
    sh2rs!("sleep 1").ok();
    simple_tray::runjs("main", r#"document.querySelector('[data-slot="sidebar.settings"]>button').click()"#);
}
