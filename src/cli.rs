use std::path::PathBuf;

use clap::builder::styling::AnsiColor;
use clap::{Parser, Subcommand};

use crate::theme;

const CLI_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(AnsiColor::Green.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Yellow.on_default());

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
#[command(about = "A Rust TOP command")]
#[command(styles = CLI_STYLING)]
pub struct Cli {
    /// Sets the theme file
    #[arg(short, long, default_value_os_t = theme::get_default_path())]
    pub theme_file: PathBuf,

    /// Print the current theme
    #[arg(short, long)]
    pub print_theme: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Outputs all the system information with an ASCII art of the OS
    Fetch {
        /// Force a logo instead of the one detected for the current OS
        #[arg(
            short,
            long,
            value_name = "LOGO",
            value_parser = clap::builder::PossibleValuesParser::new(
                crate::fetch::LOGO_NAMES.iter().copied()
            ),
        )]
        logo: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_accepts_logo_flag() {
        let args = Cli::try_parse_from(["rtop", "fetch", "-l", "windows"]).unwrap();

        assert!(matches!(
            args.command,
            Some(Commands::Fetch { logo: Some(logo) }) if logo == "windows"
        ));
    }

    #[test]
    fn fetch_rejects_unknown_logo() {
        assert!(Cli::try_parse_from(["rtop", "fetch", "--logo", "nope"]).is_err());
    }
}
