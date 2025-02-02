pub mod cli;
pub mod content;
pub mod plugins;
pub mod render;
pub mod util;
pub mod config;
pub mod init;

use cli::build::cli_build;

fn main() {
    let zite_config=config::ZiteConfig::default();
    cli_build(&zite_config);
}