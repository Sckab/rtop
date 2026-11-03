use clap::Parser;
use clap::builder::styling::AnsiColor;

const CLI_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(AnsiColor::Green.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Yellow.on_default());

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
#[command(about = "A Rust TOP command")]
#[command(styles = CLI_STYLING)]
pub struct Cli {}
