use std::{fs::{self, File}, io::{self,Read, Write}, path::{Path, PathBuf}};

use crate::config::ZiteConfig;

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

    //TODO：路径修改功能完成后修改

    let mut path_public = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_public.push("public");
    delete_files_in_directory(&path_public).unwrap();

    let mut path_md2html = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_md2html.push("template");
    path_md2html.push("post");
    path_md2html.push("md2html");
    delete_files_in_directory(&path_md2html).unwrap();
}
fn delete_files_in_directory(path:&Path) -> std::io::Result<()> {
    println!("path:{:?}",path.file_name());
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        println!("entity:{:?}",path.file_name());
        if path.is_file() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}