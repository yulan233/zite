pub mod cli;
pub mod content;
pub mod plugins;

use cli::build::cli_build;

fn main() {
    cli_build();
}
