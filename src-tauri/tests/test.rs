// 测试单元 测试函数都写这里

use simple_tauri::simple_serve;
use simple_tauri::utils::sh2rs::sh2rs;
use simple_tauri::utils::sh2rs::try_quote;

#[test]
fn on_tray_before() -> Result<(),String> {
    eprintln!("111");
    simple_serve::set_pkg("npm","@deepseek-ai/dsh");
    #[cfg(windows)]
    simple_serve::set_start_cmd!("start.bat");//DSH_HOME=%USERPROFILE%\.dsh ;../node node_modules/@deepseek-ai/dsh/lib/bin.js web
    simple_serve::set_ensure_server(|_ver,dir|{
        sh2rs!("cd {}",dir).ok();
        simple_tauri::utils::ensure_node("","")?;
        #[cfg(windows)]
        sh2rs!("echo {} >  install.bat",try_quote!("@echo off")).ok();
        sh2rs!("echo {} >> install.bat",try_quote!("set NPM_CONFIG_PREFIX=\"%~dp0/../nodejs/bin\"")).ok();
        sh2rs!("echo {} >> install.bat",try_quote!("set NPM_CONFIG_CACHE=\"%~dp0/../nodejs/cache\"")).ok();
        sh2rs!("echo {} >> install.bat",try_quote!("set PATH=\"%~dp0/../nodejs;%~dp0/../nodejs/bin;%PATH%\"")).ok();
        sh2rs!("echo {} >> install.bat",try_quote!("npm install pnpm -g")).ok();
        sh2rs!("echo {} >> install.bat",try_quote!("pnpm install @deepseek-ai/dsh")).ok();
        sh2rs!("echo {} >> install.bat",try_quote!("pnpm approve-builds -all")).ok();
        sh2rs!("echo {} >> install.bat",try_quote!("echo {} >  start.cmd",try_quote!("@echo off"))).ok();
        sh2rs!("echo {} >> install.bat",try_quote!("echo {} >> start.cmd",try_quote!("set DSH_HOME=\"%~dp0.dsh\""))).ok();
        sh2rs!("echo {} >> install.bat",try_quote!("echo {} >> start.cmd",try_quote!("./node_modules/.bin/dsh.CMD web --no-open --port 34333 --trusted-host 127.0.0.1"))).ok();
        sh2rs!("sh ./install.bat").ok();
        // install
        eprintln!("123");
        Ok(())
    });

    let _ = simple_serve::check_update(false)?;
    Ok(())
}