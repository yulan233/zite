use std::{fs::{self, File}, io::{self,Read, Write}, path::{Path, PathBuf}};

use crate::config::{self, ZiteConfig};

pub fn r_file2str(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn w_str2file(path: &Path, content: &str) -> io::Result<()> {
    // 确保目标目录存在，如果不存在则创建
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 使用 OpenOptions 来打开文件，允许写入和创建
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

    // 写入内容到文件
    file.write_all(content.as_bytes())?;

    Ok(())
}
pub fn clear_generate_public_files(zite_config:&ZiteConfig){
    let config_path=&zite_config.config_path;

    delete_files_in_directory(&config_path.public).unwrap();
    delete_files_in_directory(&config_path.template.join("post\\md2html")).unwrap();
}
fn delete_files_in_directory(path:&Path) -> std::io::Result<()> {
    println!("path:{:?}",path.file_name());
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        println!("entity:{:?}",path.file_name());
        if path.is_file() {
            fs::remove_file(path)?;
        }else if path.is_dir(){
            delete_files_in_directory(&path)?;
            fs::remove_dir(path)?;
        }
    }
    Ok(())
}