use clap::Parser;
use std::io;

mod app;
mod cli;
mod system;
mod tui;

fn main() -> anyhow::Result<()> {
    cli::Cli::parse();

    ratatui::run(|terminal| app::App::new().map_err(io::Error::other)?.run(terminal))?;

    Ok(())
}
