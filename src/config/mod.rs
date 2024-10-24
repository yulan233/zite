use comrak::Options;
use config_path::ConfigPath;
use serde::Deserialize;

pub mod config_path;

pub struct ZiteConfig<'a> {
    cormark_options: Options<'a>,
    pub config_path: ConfigPath,
}

#[derive(Deserialize)]
pub struct PostFunction {
    math_enable: bool,
}
impl<'a> ZiteConfig<'a> {
    pub fn new(public:&str,template :&str,md :&str)->Self{
        let mut cormark_options = Options::default();
        //TODO:读取zite_config文件中的配置
        let post_function = PostFunction { math_enable: true };

        //将文章的功能打开
        if post_function.math_enable {
            cormark_options.extension.math_dollars = true;
        }

        ZiteConfig {
            cormark_options,
            config_path: ConfigPath::new(public,template,md),
        }
    }
    pub fn get_cormark_options(self: &Self) -> &Options<'a> {
        &self.cormark_options
    }
}
impl Default for ZiteConfig<'_> {
    fn default() -> Self {
        ZiteConfig::new("public","template","md")
    }
}
