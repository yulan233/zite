use clap::{Parser,Subcommand};

#[derive(Parser, Debug)]
#[command(author="yulan233", version, about="hello", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[command(subcommand)]
    build:Option<Build>
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
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}