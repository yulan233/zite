pub mod cli;
pub mod content;
pub mod plugins;
pub mod render;
pub mod util;
pub mod config;

use cli::build::cli_build;

fn main() {
    cli_build();
}