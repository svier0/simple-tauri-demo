// 测试单元 测试函数都写这里

use simple_tauri::simple_serve;
use simple_tauri::utils::sh2rs::sh2rs;
// use simple_tauri::utils::sh2rs::try_quote;

#[test]
fn test_script_install() -> Result<(),String> {
    simple_serve::set_pkg("npm","@deepseek-ai/dsh");
    #[cfg(windows)]
    simple_serve::set_ensure_server(|_ver,dir|{
        sh2rs!("mkdir -p {}",dir).ok();
        sh2rs!("cd {}",dir).ok();
        // simple_tauri::utils::ensure_node("","")?;
        eprintln!("ensure_node ok");
        simple_tauri::utils::ensure_pnpm("","")?;
        eprintln!("ensure_pnpm ok");
        Ok(())
    });

    let _ = simple_serve::check_update(false)?;
    Ok(())
}