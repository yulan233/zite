use std::{collections::HashMap, error::Error};

use serde_json::value::{to_value, Value};
use tera::{try_get_value, Context, Result, Tera};

use crate::{
    config::{config_path::ConfigPath, ZiteConfig},
    util::file_about::w_str2file,
};

fn tempalte(config_path: &ConfigPath) -> Tera {
    let mut tera = match Tera::new(config_path.template.join("**\\*.html").to_str().unwrap()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    //TODO:添加所有template到post
    // tera.autoescape_on(vec!["html", ".sql"]);
    tera.add_raw_template(
        "post_render.html",
        &(r#"{% extends "post/post.html" %}
{% block post_content %}
    {% include ""#
            .to_owned()
            + "post/md2html/Zite.html"
            + r#"" %}
{% endblock post_content %}"#),
    )
    .unwrap();
    tera.register_filter("do_nothing", do_nothing_filter);

    tera
}

fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(s).unwrap())
}

pub fn render_template(zite_config: &ZiteConfig) {
    let mut context = Context::new();
    //TODO: 读取ZiteConfig中上下文配置
    context.insert("title", &"Zite");
    context.insert("math_enable", &true);
    let config_path = &zite_config.config_path;

    //TODO:批量读取所有html文件

    match tempalte(config_path).render("post_render.html", &context) {
        Ok(s) => {
            // println!("{:?}", s);
            // let mut path = zite_config.config_path.public.clone(); // 获取项目根目录
            // path.push("\\post\\Zite.html"); // 添加文件名到路径

            let _ = w_str2file(&config_path.public.join("post\\Zite.html"), &s);
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

#[cfg(test)]
mod test1 {

    #[test]
    fn test_render() {}
}
