## Objective
- 完善通用库 `simple-tauri`(0.3.0, 已发布 crates.io; GitHub: svier0/simple-tauri); demo 仓库 `simple-tauri-demo`。

## Important Details
- 用户硬约束:①测试写 `tests/`;②结论须实证(cargo test);③按描述逐字对齐;④未经许可不改库公开 API;⑤错误提示透传不写死;⑥库须通用。
- 沟通偏好:中文、简短;反感推测、绕弯、越权改文件、问太多问题;极度反感不仔细读用户指令就动手改。
- **致命教训(push 事件)**:用户说"提交"= 本地 `git commit`;绝不 push，除非用户明确说"推送"。
- **关键方法论教训**:改 bug 就改 bug 不重写;从调用处入手修复;不在回答前就改文件;只改用户指定的文件;用户问问题时只回答不动手。
- **定位问题规则**:定位=只定位不改代码;过程中可临时改文件(加debug等);定位完后恢复文件到定位前状态;只报告结果,等用户确认后再修复。
- **疑问句铁律**:用户以"询问/是不是/为什么"等疑问开头时，**禁止修改任何文件**，只与用户进行讨论回答用户问题。
- git 作者系统配置 `心衍 <vier@j7yx.com>`;无 gpg;禁止显式 set 作者。
- **宏定义位置规则**:宏一律定义在 `macros` crate 中(过程宏);`macro_rules!` 不能放 proc-macro crate。

## Work State
### Completed
- 库+macros v0.3.0 发布到 crates.io
- set_ipc_cmds! 支持 pub use 语句(提交 `6002f62`)
- wait_port 支持任意类型(提交 `6a86725`):IntoPort trait
- config::get 改为2参数: `get(key, default)`
- `get_or!` 宏:字符串字面量 → `String`;变量 `&str` → `&str`;添加 `.to_owned()`(提交 `1634cb2`)
- demo 提交 `0fadaac`: feat: config初始化 & port动态化
- demo 提交 `1a6c928`: docs: add AGENTS.md
- 回答用户: `set_window_list!` 是过程宏，`format!` 返回运行时 `String`，无法作为过程宏输入
- 库和 demo 均已推送到 origin
- 库提交 `0afe1bc`: `update.rs`/`sh.rs` 错误透传; 删除4个过时测试文件
- **Tauri IPC 问答**:后端返回 `serde_json::Value` → 前端 `invoke()` 自动反序列化为 JS 对象，无需手动 `JSON.parse`
- **库 config.rs 提交 `16e88b0`** — config模块改进:
  - `raw()` 返回值替代 Option，未初始化用 `"{}"` 兜底
  - `set()` 写入前自动创建父目录(`create_dir_all`)
  - `replace_value` 返回 `bool`，未找到 key 时追加到 JSON 对象末尾
  - `load()` 优先检查文件是否存在；`PATH.set` 提前到文件存在检查之前

### Active
- (none)

### Blocked
- (none)

## Next Move
1. 等待用户下一步需求

## Relevant Files
- `D:/x/rust/simple-tauri`(库)
- `D:/x/rust/simple-tauri/macros`(宏)
- `D:/x/rust/simple-tauri-demo`(demo)
- `D:/x/rust/tg-ws-proxy-tray` (另一个demo)