use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, Options};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn md2html(buffer:String,options:&Options)->String {
    // 返回的节点在提供的Arena中创建，并受其生命周期的约束
    let arena = Arena::new();

    // 解析文档
    let root = parse_document(
        &arena,
        &buffer,
        options,
    );

    // 遍历root的所有后代节点
    for node in root.descendants() {
        // 如果节点是math
        if let NodeValue::Math(ref mut math) = node.data.borrow_mut().value {
            if math.display_math{
                math.literal.insert_str(0, "$$");
                math.literal.push_str("$$");
            }else{
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

fn r_file2str(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
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
mod tests {

    use std::path::PathBuf;

    use super::*;
    #[test]
    fn test_md() {
        let mut options = Options::default();
        // options.extension.math_code=true;
        options.extension.math_dollars=true;
        // 构建到文件的路径
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // 获取项目根目录
        path.push("md\\Zite.md"); // 添加文件名到路径
        let text = r_file2str(&path).unwrap();
        // println!("{}", text.unwrap());
        let html=md2html(text,&options);
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // 获取项目根目录
        path.push("template\\post\\md2html\\Zite.html"); // 添加文件名到路径
        w_str2file(&path, &html).unwrap();
    }
}
