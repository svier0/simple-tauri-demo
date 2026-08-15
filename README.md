# dsh — simple-tauri 使用示例

基于 [simple-tauri](https://github.com/svier0/simple-tauri) 库的 Tauri 2 桌面应用示例。
本示例演示了快速将Deepseek-Harness打包为桌面客户端。

## 功能

- **托盘常驻**：静默启动，点击托盘图标/菜单显示主窗口
- **多窗口**：主窗口（main）、设置窗口（setting）、加载窗口（load）
- **内置服务器**：启动时拉起 `server/dsh-0.1.0`（run.bat），主窗口加载 `http://127.0.0.1:3080`
- **单实例**：Windows 互斥体，防止重复启动
- **NSIS 打包**：服务器目录作为资源一并打包（`tauri.conf.json` 中 `resources` 映射 `"../server/" → "server/"`）

## 项目结构

```
src-tauri/
├── src/
│   ├── main.rs        # 单实例互斥体 + 入口
│   └── lib.rs         # simple-tauri 库的全部配置（不到 60 行）
├── tauri.conf.json    # Tauri 配置（resources 映射打包 server/）
server/dsh-0.1.0/      # 内置服务器（run.bat 启动），打包资源，不跟踪 git
src/
├── load.html          # 启动加载页
├── setting.html       # 设置页
└── nobroder.html      # 无边框页
```

## 核心代码

`src-tauri/src/lib.rs` 全部配置：

```rust
use simple_tauri_lib::simple_tray;
use simple_tauri_lib::simple_serve;

#[tauri::command]
fn ipc_version() -> String { "1.0.0".to_string() }

#[cfg(windows)]
pub fn run() {
    simple_tray::set_ipc_cmds![ipc_version];

    // 窗口列表：[id, 标题, url, 宽, 高, 创建即显示]
    simple_tray::set_window_list!(r#"[
        ["main",    "",      "http://127.0.0.1:3080", 1280.0, 800.0],
        ["setting", "设置",   "",                        null,   null,   false],
        ["load",    "Loading", "",                      380.0,  280.0,  false],
    ]"#);

    // 托盘菜单：[id, 标签, 回调]，id 为空为分隔线
    simple_tray::set_tray_menu!(r#"[
        ["show", "显示主界面"],
        ["show-setting", "设置", "show_setting"],
        [],
        ["light"],
    ]"#);

    simple_tray::hooks!(on_tray_before, on_tray_after, on_quit);
    simple_tray::run!();
}

// 托盘创建前：显示加载页 → 启动服务器 → 等待端口 → 关闭加载页
fn on_tray_before() -> Result<(), String> {
    simple_tray::show_window("load");
    simple_serve::start("server/dsh-0.1.0", "run.bat")
        .map_err(|e| format!("服务器启动失败: {e}"))?;
    simple_serve::wait_port(3080)
        .map_err(|e| format!("服务器启动超时: {e}"))?;
    simple_tray::close_window("load");
    Ok(())
}

// 托盘创建后：显示主窗口
fn on_tray_after() -> Result<(), String> {
    simple_tray::show_window("main");
    Ok(())
}

// 退出时停止服务器
fn on_quit() -> Result<(), String> {
    simple_serve::stop();
    Ok(())
}

// 菜单回调（无参，内部用全局 AppHandle）
fn show_setting() {
    simple_tray::show_window("main");
    simple_tray::runjs("main", r#"document.querySelector('[data-slot="sidebar.settings"]>button').click()"#);
}
```

## 开发运行

```bash
# 依赖 simple-tauri 库（本地路径或 crates.io）
cargo build --release
cp -f target/release/dsh.exe target/dsh.exe
./target/dsh.exe
```

> 注意：必须用 release 构建运行，debug 构建会弹出黑色控制台窗口。

## 打包

```bash
cargo tauri build
```

生成 NSIS 安装包，服务器目录作为资源随包分发，安装后按 exe 所在目录解析 `server/dsh-0.1.0`（打包时通过 `"../server/" → "server/"` 映射，路径与源码目录一致）。

## 依赖

- [simple-tauri](https://github.com/svier0/simple-tauri) — 托盘/多窗口/服务器管理封装库
