use std::path::PathBuf;

use comrak::Options;
use serde::Deserialize;

mod path_about;

pub struct ZiteConfig<'a> {
    cormark_options: Options<'a>,
    pub config_path:ConfigPath
}
pub struct ConfigPath{
    pub public:PathBuf,
    pub template:PathBuf,
    pub md:PathBuf
}
#[derive(Deserialize)]
pub struct PostFunction {
    math_enable: bool,
}
impl<'a> ZiteConfig<'a> {
    pub fn new() -> Self {
        let mut cormark_options=Options::default();
        //TODO:读取zite_config文件中的配置
        let post_function=PostFunction{ math_enable: true };

        //将文章的功能打开
        if post_function.math_enable{
            cormark_options.extension.math_dollars=true;
        }


        ZiteConfig {
            cormark_options,
            config_path:ConfigPath{
                public: PathBuf::from(env!("CARGO_MANIFEST_DIR").to_owned()+"\\public"),
                template: PathBuf::from(env!("CARGO_MANIFEST_DIR").to_owned()+"\\template"),
                md: PathBuf::from(env!("CARGO_MANIFEST_DIR").to_owned()+"\\md"),
            }
        }
    }
    pub fn get_cormark_options(self:&Self)->&Options<'a>{
        &self.cormark_options
    }
}
