
use simple_tauri::config;

/// 默认配置（JSONC）
const DEFAULT: &str = r#"
{
	// 开机自启
    "auto_start": false,
    // 自动运行
    "auto_run": false,
    // 静默启动
    "silent_launch": false,
    // 自动更新
    "auto_update": false,
    // 端口
    "port": 3080,
    // 数据目录
    "dsh_home": "",
}
"#;

const PATH: &str = "data/config.json";

/// 初始化配置文件
pub fn init(){
	config::set_default(DEFAULT);
	config::load(PATH).expect("");
    if config::get_or!("dsh_home","".to_string()).is_empty() {
        let dsh_home = std::env::var_os("DSH_HOME")
        	.or_else(||{
        		let user_home: std::path::PathBuf = std::env::var_os("USERPROFILE")
				    .or_else(|| std::env::var_os("HOME"))
				    .map(std::path::PathBuf::from)
				    .unwrap_or_else(|| simple_tauri::simple_tray::resource_dir(""));
        		Some(user_home.join(".dsh").into_os_string())
        	})
        	.map(|s| s.to_string_lossy().to_string())
		    .unwrap_or_default();
        let _ = config::set("dsh_home",dsh_home);
    }
}

pub fn get_port() -> i64 {
    config::get_or!("port",3080)
}