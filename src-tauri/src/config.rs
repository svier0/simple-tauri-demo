
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
    if config::get_str("dsh_home").unwrap_or_default().is_empty() {
        let dsh_home = "~/.dsh";
        let _ = config::set("dsh_home",dsh_home);
    }
}