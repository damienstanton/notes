#![allow(unused)]
use anyhow::Result;
use notebook::flyer;

/// Use this module to specify the CLI cmd structure.
mod cmd {
    use clap::Parser;
    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub(crate) struct App {
        #[arg(long, short)]
        /// true or false
        boolean: bool,
    }

    pub(crate) fn new_app() -> App {
        App::parse()
    }
}

fn main() -> Result<()> {
    let _ = cmd::new_app();
    let _ = flyer::run_flyer()?;
    Ok(())
}
