// 测试单元 测试函数都写这里

use simple_tauri::utils::wget;

#[test]
fn test_wget_downloads_file() {
    // 真实联网下载，验证 wget 走 ureq 能成功落盘到临时目录
    // URL 以 / 结尾时文件名回退为 index.html
    let url = "https://www.baidu.com/";
    let r = wget(url);
    assert!(
        r.is_ok(),
        "wget({}) 应成功下载，实际错误: {:?}",
        url,
        r.err()
    );
}
