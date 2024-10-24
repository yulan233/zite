use std::path::PathBuf;

pub struct ConfigPath{
    pub public:PathBuf,
    pub template:PathBuf,
    pub md:PathBuf
}
impl ConfigPath{
    pub fn new(public:&str,template:&str,md:&str)->Self{
        ConfigPath{
            public: PathBuf::from(env!("CARGO_MANIFEST_DIR").to_owned()+"\\"+public),
            template: PathBuf::from(env!("CARGO_MANIFEST_DIR").to_owned()+"\\"+template),
            md: PathBuf::from(env!("CARGO_MANIFEST_DIR").to_owned()+"\\"+md),
        }
    }
}
impl Default for ConfigPath{
    fn default() -> Self {
        ConfigPath::new("public","template","md")
    }
}