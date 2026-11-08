use clap::Parser;
use std::io;

mod cli;
mod system;
mod tui;

fn main() -> anyhow::Result<()> {
    cli::Cli::parse();

    ratatui::run(|terminal| tui::App::new().map_err(io::Error::other)?.run(terminal))?;

    Ok(())
}
