use simple_tauri::simple_tray;
use simple_tauri::simple_serve;
use simple_tauri::utils::sh2rs::sh2rs;
use indoc::indoc;

mod ipc;
mod config;
mod init;

pub fn run() {
    simple_tray::run!();
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

fn show_cmd() {
    let dsh_home = config::get_or!("dsh_home","");
    let local_ver = simple_serve::get_local_ver();
    let work_dir = simple_serve::get_work_dir(Some(&local_ver));
    #[cfg(windows)]
    simple_tauri::utils::show_env_cmd(&format!(indoc! {r#"
            set "DSH_HOME={}"
            set "PATH={}/node_modules/.bin;%PATH%"
            cls
            dsh --help
        "#},dsh_home,work_dir));
    #[cfg(not(windows))]
    simple_tauri::utils::show_env_cmd(&format!(indoc! {r#"
            export DSH_HOME="{}"
            export PATH="{}/node_modules/.bin:$PATH"
            clear
            dsh --help
        "#},dsh_home,work_dir));
}