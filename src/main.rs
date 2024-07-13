// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;

use std::io::{Read, Seek, Write};
use std::{env, fs};
use std::path::PathBuf;

use hex;
use sysinfo::{System};
use tauri::{Manager,WindowBuilder, WindowUrl};
use std::sync::{Arc, Mutex};

use tokio::time::{self,Duration};

use core::archived::check_save_path;
use core::archived::compress_save;
use once_cell::sync::Lazy;
use walkdir::WalkDir;
use core::setting::get_app_path;
use core::archived::unzip_to_folder;
static ELDEN_RING_SAVE_PATH: Lazy<Mutex<Result<PathBuf, bool>>> = Lazy::new(|| {
    Mutex::new(check_save_path())
});

static AUTO_SAVE_SETTING: Lazy<Mutex<bool>> = Lazy::new(|| {
    Mutex::new(true)
});

#[tokio::main]
async fn main() {
    // 创建并配置主窗口
    let context = tauri::generate_context!();
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_save_path, auto_save, find_save_archived, manual_save, unzip_file, zip_info, delete_save, change_save_status])
        .setup(|app| {
            let window = tauri::WindowBuilder::new(
                app,
                "main",
                tauri::WindowUrl::App("/".into())
            )
                .inner_size(600.0, 600.0)
                .title("Elden Ring Save Manager")
                .decorations(false)
                .resizable(false)
                .build()
                .expect("error while building window");

            // 获取窗口关联的监视器信息
            if let Ok(Some(monitor)) = window.current_monitor() {
                let monitor_size = monitor.size();
                let x = (monitor_size.width as f64 - 600.0) / 2.0;
                let y = (monitor_size.height as f64 - 600.0) / 2.0;

                // 设置窗口居中
                window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y })).unwrap();
            } else {
                println!("No monitor information available or an error occurred.");
            }

            Ok(())
        })
        .build(context)
        .expect("error while running tauri application");
    // 创建共享的系统状态
    let system = Arc::new(Mutex::new(System::new_all()));
    let app_handle_clone = app.handle().clone();
    let system_clone = Arc::clone(&system);
    tauri::async_runtime::spawn(async move {
        let mut application_has_run = false;
        loop {

            let process_exists = {
                let mut system = system_clone.lock().unwrap();
                system.refresh_all();
                system.processes().values().any(|p| p.name().contains("eldenring"))
            };

            if process_exists {
                application_has_run = true;
                println!("eldenring.exe is running");
            } else if !process_exists && application_has_run {
                // } else if !process_exists { //debug
                application_has_run = false;
                println!("eldenring.exe is not running");
                let is_auto_save = AUTO_SAVE_SETTING.lock().unwrap();
                if *is_auto_save{
                    create_notification_window(app_handle_clone.clone());
                }
            }
            time::sleep(Duration::from_secs(3)).await;
        }
    });
    app.run(|_, _| {});
}

#[tauri::command]
fn auto_save() -> String{
    let save_path = ELDEN_RING_SAVE_PATH.lock().unwrap();
    match &*save_path {
        Ok(path) => {
            compress_save(path,"auto").unwrap_or_else(|e| format!("Compression failed: {}", e))
        },
        Err(_) => "Can't compress, cause can't find Elden Ring Save Path".to_string(),  // 如果找不到保存路径，返回错误信息
    }
}

#[tauri::command]
fn manual_save() -> String{
    let save_path = ELDEN_RING_SAVE_PATH.lock().unwrap();
    match &*save_path {
        Ok(path) => {
            compress_save(path,"manual").unwrap_or_else(|e| format!("Compression failed: {}", e))
        },
        Err(_) => "Can't compress, cause can't find Elden Ring Save Path".to_string(),  // 如果找不到保存路径，返回错误信息
    }
}

#[tauri::command]
fn get_save_path() -> String{
    let save_path = ELDEN_RING_SAVE_PATH.lock().unwrap();
    match &*save_path {
        Ok(path) => path.to_str().unwrap_or("error").to_string(),
        Err(_) => "can't find".to_string(),
    }
}

#[tauri::command]
fn delete_save(path:String) {
    fs::remove_file(path).expect("TODO: panic message");
    // println!("执行了删除")
}
#[tauri::command]
fn unzip_file(path:String) {
    let save_path = ELDEN_RING_SAVE_PATH.lock().unwrap();
    match &*save_path {
        Ok(save_path) => {
            unzip_to_folder(path,save_path).unwrap()
        },
        Err(e) => eprintln!("{}", e)
    }
}
#[tauri::command]
fn zip_info(path:String){
    println!("点击了查看存档信息：{}",path)
}

#[tauri::command]
fn change_save_status(status:bool){
    let mut v = AUTO_SAVE_SETTING.lock().unwrap();
    *v = status;
}
fn create_notification_window(app: tauri::AppHandle) {
    // 尝试从 app 中获取主窗口
    if let Some(window) = app.get_window("main") {
        // 尝试获取当前监视器信息，并处理 Result
        match window.current_monitor() {
            Ok(Some(monitor)) => { // 当成功获取到监视器时
                let monitor_size = monitor.size();
                let window_width = 450.0;
                let window_height = 170.0;
                let x_position = monitor_size.width as f64 - window_width;
                let y_position = monitor_size.height as f64 - window_height;
                let _notification_window = WindowBuilder::new(
                    &app,
                    "notification",
                    WindowUrl::App("/notification".into())
                )
                    .title("Notification")
                    .decorations(false)
                    .resizable(false)
                    .inner_size(window_width, window_height)
                    .position(x_position, y_position)
                    .build()
                    .expect("failed to create window");
            },
            Ok(None) => {
                println!("No monitor found for the main window");
            },
            Err(e) => {
                println!("Error retrieving monitor information: {}", e);
            }
        }
    } else {
        println!("Main window not found");
    }
}

#[tauri::command]
fn find_save_archived() -> Vec<String> {
    let mut zip_files = Vec::new();
    let app_path = get_app_path();
    for entry in WalkDir::new(app_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "zip") {
            zip_files.push(path.to_string_lossy().into_owned());
        }
    }
    zip_files
}

