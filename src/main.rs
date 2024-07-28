pub mod cli;
pub mod content;
pub mod plugins;
pub mod render;

use cli::build::cli_build;

fn main() {
    cli_build();
}