//! This module is here in case we want to extend the CLI interface.
use clap::Parser;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct App {
    #[arg(long, short)]
    /// true or false
    boolean: bool,
}

pub fn new_app() -> App {
    App::parse()
}
