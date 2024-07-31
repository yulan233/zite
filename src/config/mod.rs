use comrak::Options;
use serde::{Deserialize, Serialize};

mod path_about;
//TODO：完善配置功能，路径功能
pub struct ZiteConfig<'a> {
    cormark_options: Options<'a>,
}

#[derive(Deserialize)]
pub struct PostFunction {
    math_enable: bool,
}
impl<'a> ZiteConfig<'a> {
    pub fn new() -> Self {
        let mut cormark_options=Options::default();
        let post_function=PostFunction{ math_enable: true };

        //将文章的功能打开
        if post_function.math_enable{
            cormark_options.extension.math_dollars=true;
        }


        ZiteConfig {
            cormark_options
        }
    }
    pub fn get_cormark_options(self:&Self)->&Options<'a>{
        &self.cormark_options
    }
}
