use std::{
    collections::HashMap,
    error::Error,
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde_json::value::{to_value, Value};
use tera::{try_get_value, Context, Result, Tera};

pub fn tempalte() -> Tera {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("template\\**\\*.html");

    let mut tera = match Tera::new(path.to_str().unwrap()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    // tera.autoescape_on(vec!["html", ".sql"]);
    tera.add_raw_template(
        "post_render.html",
        &(r#"{% extends "post/post.html" %}
{% block post_content %}
    {% include ""#.to_owned()+"post/md2html/Zite.html"+r#"" %}
{% endblock post_content %}"#),
    ).unwrap();
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
    use std::{
        error::Error,
        path::{self, PathBuf},
    };

    use tera::{Context, Tera};

    use crate::render::render::w_str2file;

    use super::tempalte;

    #[test]
    fn test_render() {
        let mut context = Context::new();
        context.insert("title", &"Zite");
        context.insert("math_enable", &true);
        // context.insert("bio", &"<script>alert('pwnd');</script>".to_string());

        // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // path.push("src\\template\\user\\profile.html");
        match tempalte().render("post_render.html", &context) {
            Ok(s) => {
                println!("{:?}", s);
                let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // 获取项目根目录
                path.push("public\\Zite.html"); // 添加文件名到路径

                let _ = w_str2file(&path, &s);
            }
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
