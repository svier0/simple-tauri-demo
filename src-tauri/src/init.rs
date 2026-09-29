use simple_tauri::simple_tray;
use simple_tauri::simple_serve;
use simple_tauri::utils::sh2rs::sh2rs;
use indoc::indoc;
use super::config;

fn show_load_tips(s: &str){
    let silent_launch = config::get_or!("silent_launch",false);
    if silent_launch { return; }
    simple_tray::runjs("load", &format!("document.querySelector('p.tips').innerHTML='{}'",s));
}

#[cfg(windows)]
fn run_script_install(port: i64,dsh_home: &str,dir: &str) -> Result<(),String> {
    simple_tauri::utils::run_env_cmd(&indoc! {r#"
            call pnpm init
            call pnpm install @deepseek-ai/dsh
            call pnpm approve-builds -all
        "#}).ok();
    simple_tauri::utils::write_env_script(&format!(indoc! {r#"
            set "DSH_HOME={}"
            set "PATH={}/node_modules/.bin;%PATH%"
            dsh web --no-open --port {} --trusted-host 127.0.0.1
        "#},dsh_home,dir,port),"start.bat")?;
    Ok(())
}

#[cfg(not(windows))]
fn run_script_install(port: i64,dsh_home: &str,dir: &str) -> Result<(),String> {
    simple_tauri::utils::run_env_cmd(&indoc! {r#"
            pnpm init
            pnpm install @deepseek-ai/dsh
            pnpm approve-builds -all
        "#}).ok();
    simple_tauri::utils::write_env_script(&format!(indoc! {r#"
            export DSH_HOME="{}"
            export PATH="{}/node_modules/.bin:$PATH"
            dsh web --no-open --port {} --trusted-host 127.0.0.1
        "#},dsh_home,dir,port),"start.sh")?;
    Ok(())
}

// 托盘创建前回调
pub fn on_tray_before() -> Result<(), String> {
    // 读取配置
    let silent_launch = config::get_or!("silent_launch",false);
    let auto_run      = config::get_or!("auto_run",false);
    let dsh_home      = config::get_or!("dsh_home","");
    let port          = config::port();

    // 显示加载窗口
    if !silent_launch {
        simple_tray::show_window("load");
        sh2rs!("sleep 1").ok();
    }

    // 启动服务时执行的命令
    #[cfg(windows)]
    simple_serve::set_start_cmd!("start.bat");
    #[cfg(not(windows))]
    simple_serve::set_start_cmd!("start.sh");

    // 需要更新服务端时的回调
    simple_serve::set_ensure_server(move |ver,dir|{
        sh2rs!("mkdir -p {}",dir).ok();
        sh2rs!("cd {}",dir).ok();
        show_load_tips("安装dsh");
        run_script_install(port,&dsh_home,&dir)?;
        sh2rs!("echo {} > {}",ver,"version.txt").ok();
        Ok(())
    });

    // 检查版本更新
    show_load_tips("检测运行环境");
    simple_serve::auto_check_update()?;

    if auto_run {
        // 启动服务
        show_load_tips("服务启动中");
        simple_serve::start()
            .map_err(|e| format!("服务器启动失败: {e}"))?;
        show_load_tips("服务启动中 20%");
        sh2rs!("sleep 1").ok();
        // 检测端口拉起成功才返回继续走托盘创建逻辑
        simple_serve::wait_port(port)
            .map_err(|e| format!("服务器启动超时: {e}"))?;
        show_load_tips("服务启动中 60%");
        sh2rs!("sleep 2").ok();
        show_load_tips("服务启动中 80%");
        // 获取主窗口url logs中找出"dsh web: "开头的行截取出后面的链接
        let logs = simple_serve::get_log();
        let url = logs.lines()
            .find(|l| l.starts_with("dsh web: "))
            .map(|l| l[9..].to_string())
            .unwrap_or_default();
        simple_tray::edit_window_item("main", |w| { w.url = url.to_string(); })?;
        show_load_tips("服务启动中 100%");
    }

    if !silent_launch {
        // 关闭加载窗口
        simple_tray::close_window("load");
        // 显示主窗口
        simple_tray::show_window("main");
    }
    Ok(())
}