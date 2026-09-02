// 测试单元 测试函数都写这里

use simple_tauri::simple_serve;
use simple_tauri::utils::sh2rs::sh2rs;
use simple_tauri::utils::sh2rs::try_quote;
use indoc::indoc;

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
    simple_serve::set_start_cmd!("start.bat");
    Ok(())
}
#[test]
fn test_script_install() -> Result<(),String> {
    let port = 34333;
    let server_home = "";
    simple_serve::set_pkg("npm","@deepseek-ai/dsh");
    simple_serve::set_ensure_server(|_ver,dir|{
        sh2rs!("mkdir -p {}",dir).ok();
        sh2rs!("cd {}",dir).ok();
        // show_load_tips("安装node");
        simple_tauri::utils::ensure_node("","")?;
        // show_load_tips("安装pnpm");
        simple_tauri::utils::ensure_pnpm("","")?;
        // show_load_tips("安装dsh");
        run_script_install(port,server_home)?;
        Ok(())
    });

    let _ = simple_serve::check_update(false)?;
    Ok(())
}