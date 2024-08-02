use comrak::nodes::NodeValue;
use comrak::{format_html, parse_document, Arena, Options};

use crate::config::ZiteConfig;
use crate::util::file_about::{r_file2str, w_str2file};

fn md2html(buffer: String, options: &Options) -> String {
    // 返回的节点在提供的Arena中创建，并受其生命周期的约束
    let arena = Arena::new();

    // 解析文档
    let root = parse_document(&arena, &buffer, options);

    // 遍历root的所有后代节点
    for node in root.descendants() {
        // 如果节点是math
        if let NodeValue::Math(ref mut math) = node.data.borrow_mut().value {
            if math.display_math {
                math.literal.insert_str(0, "$$");
                math.literal.push_str("$$");
            } else {
                math.literal.insert_str(0, "$");
                math.literal.push_str("$");
            }
        }
    }

    // 创建一个空的Vec，用于存储转换后的HTML
    let mut html = vec![];

    // 将root转换为HTML，并存储到html中
    format_html(root, &Options::default(), &mut html).unwrap();
    String::from_utf8(html).unwrap()
}

pub fn content_generate(zite_config:&ZiteConfig) {
    let options =zite_config.get_cormark_options();
    let config_path=&zite_config.config_path;

    // 读取md文件
    // let mut path = zite_config.config_path.md.clone(); 
    // path.push("\\Zite.md"); 
    let text = r_file2str(&config_path.md.join("Zite.md")).unwrap();
    
    // 转换md到html并写入
    let html = md2html(text, options);
    // let mut path = zite_config.config_path.template.clone(); 
    // path.push("\\post\\md2html\\Zite.html"); 
    w_str2file(&config_path.template.join("post\\md2html\\Zite.html"), &html).unwrap();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::ZiteConfig;



    #[test]
    fn test_md() {
        let p=ZiteConfig::new();
        let pa=p.config_path;

        println!("{:?}",pa.md.join("Zite.md"));
    }
}
