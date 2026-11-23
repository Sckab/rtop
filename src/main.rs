use clap::Parser;

use crate::theme::Theme;

mod app;
mod cli;
mod fetch;
mod system;
mod theme;
mod tui;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    let theme = match std::fs::read_to_string(&args.theme_file) {
        Ok(contents) => toml::from_str(&contents)?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Theme::default(),
        Err(error) => return Err(error.into()),
    };

    if args.print_theme {
        theme.print();

        return Ok(());
    }

    if let Some(command) = args.command {
        match command {
            cli::Commands::Fetch { logo } => {
                fetch::fetch(theme, logo.as_deref())?;
            }
        }
    } else {
        let stats_system = sysinfo::System::new()?;
        let process_system = sysinfo::System::new()?;

        let users = sysinfo::Users::new_with_refreshed_list()?;
        let components = sysinfo::Components::new_with_refreshed_list()?;

        let disks = sysinfo::Disks::new_with_refreshed_list()?;
        let network = sysinfo::Networks::new_with_refreshed_list()?;

        ratatui::run(|terminal| {
            app::App::new().run(
                theme,
                terminal,
                stats_system,
                process_system,
                components,
                disks,
                network,
                users,
            )
        })?;
    }

    Ok(())
}
