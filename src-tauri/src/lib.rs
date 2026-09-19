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
    let server_home   = config::get_or!("server_home","");
    #[cfg(windows)]
    simple_tauri::utils::show_env_cmd(&format!(indoc! {r#"
            set "DSH_HOME={}"
            set "PATH=%DSH_HOME%/node_modules/.bin;%PATH%"
            cls
            cd ~
            dsh --help
        "#},server_home));
    #[cfg(not(windows))]
    simple_tauri::utils::show_env_cmd(&format!(indoc! {r#"
            export DSH_HOME="{}"
            export PATH="$DSH_HOME/node_modules/.bin:$PATH"
            clear
            cd ~
            dsh --help
        "#},server_home));
}