use std::path::{PathBuf};
use std::fs;
use std::io;
use dirs::home_dir;
use crate::auto_save;

pub fn get_app_path() -> PathBuf {
    // 尝试获取主目录
    let home = home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    // 构造 .ersk 文件夹的路径
    let app_path = home.join(".ersk");
    // 尝试创建 .ersk 目录，忽略可能的错误
    if !app_path.exists() {
        // 创建目录，并检查是否成功
        let _ = fs::create_dir_all(&app_path);
        auto_save();
    }
    // 总是返回一个路径，即使它可能没有被创建
    app_path
}