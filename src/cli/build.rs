use clap::{ Parser, Subcommand};

use crate::{config::ZiteConfig, content::content_generate, render::render::render_template, util::file_about::clear_generate_public_files};

#[derive(Parser, Debug)]
#[command(author="yulan233", version, about="hello", long_about = None)]
struct Cli {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[command(subcommand)]
    build:Build
}

#[derive(Subcommand,Debug)]
enum Build{
    Generate,
    Clear,
    ///local server
    Server
}

///构建命令行
pub fn cli_build(zite_config:&ZiteConfig){
    let cli = Cli::parse();
    for _ in 0..cli.count {
        println!("Hello world!");
    }
    match &cli.build{
        Build::Generate=>generate(zite_config),
        Build::Clear => clear(zite_config),
        Build::Server => server(zite_config),
    }
}
fn generate(zite_config:&ZiteConfig){
    content_generate(zite_config);
    render_template(zite_config);
}
fn clear(zite_config:&ZiteConfig){
    print!("clear");
    clear_generate_public_files(zite_config);
}
fn server(zite_config:&ZiteConfig){
    //TODO：启动本地服务器
    todo!()
}