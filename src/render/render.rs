use std::{collections::HashMap, error::Error, io::{self, Write}, path::{Path, PathBuf}};

use serde_json::value::{to_value, Value};
use tera::{try_get_value, Context, Result, Tera};

pub fn tempalte() -> Tera {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src\\template\\**\\*");

    let mut tera = match Tera::new(path.to_str().unwrap()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec!["html", ".sql"]);
    tera.register_filter("do_nothing", do_nothing_filter);
    tera
}

pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(s).unwrap())
}

fn w_str2file(path: &Path, content: &str) -> io::Result<()> {
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

#[cfg(test)]
mod test1 {
    use std::{error::Error, path::{self, PathBuf}};

    use tera::{Context, Tera};

    use crate::render::render::w_str2file;

    use super::tempalte;

    #[test]
    fn test_render() {
        let mut context = Context::new();
        context.insert("username", &"Bob");
        context.insert("numbers", &vec![1, 2, 3]);
        context.insert("show_all", &false);
        // context.insert("bio", &"<script>alert('pwnd');</script>".to_string());

        // A one off template
        Tera::one_off("hello", &Context::new(), true).unwrap();

        // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // path.push("src\\template\\user\\profile.html");
        match tempalte().render("user/profile.html", &context) {
            Ok(s) => {
                println!("{:?}", s);
                let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // 获取项目根目录
                path.push("src\\public\\profile.html"); // 添加文件名到路径

                let _ = w_str2file(&path, &s);
            },
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = e.source();
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
            }
        };
    }
}
