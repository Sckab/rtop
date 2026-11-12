use clap::Parser;

mod app;
mod cli;
mod system;
mod tui;

fn main() -> anyhow::Result<()> {
    cli::Cli::parse();

    let stats_system = sysinfo::System::new()?;
    let process_system = sysinfo::System::new()?;

    let users = sysinfo::Users::new_with_refreshed_list()?;

    let components = sysinfo::Components::new_with_refreshed_list()?;

    ratatui::run(|terminal| {
        app::App::new().run(terminal, stats_system, process_system, components, users)
    })?;

    Ok(())
}
