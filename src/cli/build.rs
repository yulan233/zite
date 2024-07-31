use clap::{ Parser, Subcommand};

use crate::{content::content_generate, render::render::render_template, util::file_about::clear_generate_public_files};

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
    ///构建
    Generate,
    ///清除
    Clear,
    ///本地服务器
    Server
}

///构建命令行
pub fn cli_build(){
    let cli = Cli::parse();
    for _ in 0..cli.count {
        println!("Hello world!");
    }
    match &cli.build{
        Build::Generate=>generate(),
        Build::Clear => clear(),
        Build::Server => server(),
    }
}
fn generate(){
    content_generate();
    render_template();
}
fn clear(){
    print!("clear");
    clear_generate_public_files();
}
fn server(){

}