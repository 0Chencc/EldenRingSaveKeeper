use std::fs::{self, File};
use std::{env, io};
use std::io::{BufReader, BufWriter};
use std::path::{Path,PathBuf};
use dirs::home_dir;
use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod, ZipWriter,ZipArchive};
use chrono::{Local, DateTime};
use crate::auto_save;

pub fn compress_save(folder_path: &Path, mode: &str) -> io::Result<String> {
    let mut newest_folder: Option<PathBuf> = None;
    let mut newest_mod_time: Option<DateTime<Local>> = None;

    // 遍历 folder_path 下的所有条目以找到最新的文件夹
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let metadata = fs::metadata(&path)?;
            let modified = metadata.modified()?.into();
            if newest_mod_time.is_none() || modified > newest_mod_time.unwrap() {
                newest_mod_time = Some(modified);
                newest_folder = Some(path);
            }
        }
    }

    match newest_folder {
        Some(folder) => {
            let mod_time = Local::now();
            let date_str = mod_time.format("%Y-%m-%d %H_%M_%S").to_string();
            let folder_name = folder.file_name().unwrap().to_str().unwrap();
            let zip_name = format!("{}-{}_{}.zip", folder_name, date_str, mode);

            let home_dir = dirs::home_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Cannot find home directory"))?;
            let ersk_dir = home_dir.join(".ersk");
            fs::create_dir_all(&ersk_dir)?;

            let output_file_path = ersk_dir.join(&zip_name);
            let file = File::create(&output_file_path)?;
            let mut zip = ZipWriter::new(file);

            let options: FileOptions<'_, ()> = FileOptions::default()
                .compression_method(CompressionMethod::Deflated)
                .large_file(false)
                .unix_permissions(0o755);

            // 压缩包含顶级目录的完整路径
            let top_folder_name = folder.file_name().unwrap().to_str().unwrap();
            let folder_prefix = folder.parent().unwrap();

            for entry in WalkDir::new(&folder) {
                let entry = entry?;
                let path = entry.path();
                let name = path.strip_prefix(folder.parent().unwrap())
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to strip prefix"))?
                    .to_str()
                    .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid UTF-8 sequence"))?;

                if path.is_file() {
                    zip.start_file(name, options)?;
                    let mut f = File::open(path)?;
                    io::copy(&mut f, &mut zip)?;
                } else if !name.is_empty() { // 直接检查字符串是否为空
                    zip.add_directory(format!("{}/", name), options)?;
                }
            }


            zip.finish()?;
            Ok(output_file_path.to_string_lossy().into_owned())
        },
        None => Err(io::Error::new(io::ErrorKind::NotFound, "No folders found in the directory"))
    }
}

pub fn check_save_path() -> Result<PathBuf, bool> {
    let local_app_data = env::var("APPDATA");
    let folder_path = match local_app_data {
        Ok(path) => PathBuf::from(path).join("EldenRing"),
        Err(_) => {
            let home_path = home_dir().ok_or(false)?;
            home_path.join("AppData").join("Roaming").join("EldenRing")
        }
    };
    if folder_path.exists() {
        Ok(folder_path)
    } else {
        Err(false)
    }
}

pub fn unzip_to_folder(zip_path: String, folder_path: &Path) -> io::Result<()> {
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(BufReader::new(zip_file))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = match file.enclosed_name() {
            Some(path) => folder_path.join(path),
            None => continue,  // Skip files with invalid names
        };

        if file.is_dir() {
            std::fs::create_dir_all(&file_path)?;
        } else {
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut output_file = File::create(&file_path)?;
            let mut writer = BufWriter::new(output_file);
            io::copy(&mut file, &mut writer)?;
        }
    }
    Ok(())
}
