use simple_tauri::simple_tray;
use simple_tauri::simple_serve;
use simple_tauri::utils::sh2rs::sh2rs;
use simple_tauri::utils::sh2rs::try_quote;

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
    // simple_tray::run!();
}

fn show_load_tips(s: &str){
    simple_tray::runjs("load", &format!("document.querySelector('p.tips').innerHTML='{}'",s));
}

#[cfg(windows)]
fn run_script_install(){
    sh2rs!("echo {} >  install.bat",
        try_quote!("@echo off")).ok();
    sh2rs!("echo {} >> install.bat",
        try_quote!("set PATH=\"%~dp0/../nodejs;%~dp0/../nodejs/bin;%PATH%\"")).ok();
    sh2rs!("echo {} >> install.bat",
        try_quote!("pnpm install @deepseek-ai/dsh")).ok();
    sh2rs!("echo {} >> install.bat",
        try_quote!("pnpm approve-builds -all")).ok();
    sh2rs!("echo {} >> install.bat",
        try_quote!("echo {} >  start.cmd",
        try_quote!("@echo off"))).ok();
    sh2rs!("echo {} >> install.bat",
        try_quote!("echo {} >> start.cmd",
        try_quote!("set DSH_HOME=\"%~dp0.dsh\""))).ok();
    sh2rs!("echo {} >> install.bat",
        try_quote!("echo {} >> start.cmd",
        try_quote!("./node_modules/.bin/dsh.CMD web --no-open --port 34333 --trusted-host 127.0.0.1"))).ok();
    sh2rs!("sh ./install.bat").ok();
    sh2rs!("rm ./install.bat").ok();
}

#[cfg(not(windows))]
fn run_script_install(){
    sh2rs!("echo {} >  install.sh",
        try_quote!("#!/bin/bash")).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("PNPM_HOME=$(pwd)/pnpm")).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("PNPM_BIN=$PNPM_HOME/pnpm.exe")).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("export PATH=\"$PNPM_HOME/bin:$(pwd)/node.exe:$PATH\"")).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("pnpm install @deepseek-ai/dsh || true")).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("pnpm approve-builds -all")).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("echo {} >  start.sh",
        try_quote!("#!/bin/bash"))).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("echo {} >> start.sh",
            try_quote!("export DSH_HOME=\"$(cd {} && pwd)/.dsh\"",
                try_quote!("$(dirname \"${{BASH_SOURCE[0]}}\")")))).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("echo {} >> start.sh",
            try_quote!("export PATH=\"./node_modules/.bin:$PATH\""))).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("echo {} >> start.sh",
            try_quote!("dsh web --no-open --port 34333 --trusted-host 127.0.0.1"))).ok();
    sh2rs!("echo {} >> install.sh",
        try_quote!("chmod 755 ./start.sh")).ok();
    sh2rs!("sh ./install.sh").ok();
    sh2rs!("rm ./install.sh").ok();
}

// 托盘创建前回调
fn on_tray_before() -> Result<(), String> {
    // 显示加载窗口
    simple_tray::show_window("load");
    sh2rs!("sleep 1").ok();
    // 设置参数
    let autoupdate = false;
    // let port = 3080;

    //
    simple_serve::set_pkg("npm","@deepseek-ai/dsh");
    #[cfg(windows)]
    simple_serve::set_start_cmd!("start.bat");//DSH_HOME=%USERPROFILE%\.dsh ;../node node_modules/@deepseek-ai/dsh/lib/bin.js web
    #[cfg(not(windows))]
    simple_serve::set_start_cmd!("start.sh");//DSH_HOME=%USERPROFILE%\.dsh ;../node node_modules/@deepseek-ai/dsh/lib/bin.js web
    simple_serve::set_ensure_server(|_ver,dir|{
        sh2rs!("cd {}",dir).ok();
        show_load_tips("安装Node");
        simple_tauri::utils::ensure_node("","")?;
        run_script_install();
        // install
        Ok(())
    });
    if autoupdate { simple_serve::enable_auto_update(); }

    // // 检查版本更新
    // show_load_tips("检测本地服务版本");
    // let _ = simple_serve::check_update(false)?;
    // // 启动服务
    // show_load_tips("服务启动中");
    // simple_serve::start()
    //     .map_err(|e| format!("服务器启动失败: {e}"))?;
    // // 检测端口拉起成功才返回继续走托盘创建逻辑
    // simple_serve::wait_port(port)
    //     .map_err(|e| format!("服务器启动超时: {e}"))?;

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
