<div align="center">

# RTOP

_A Rust TOP_

[![Rust](https://img.shields.io/badge/rust-1.95%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue)](LICENSE)

![showcase](./screenshots/showcase-1.png)

RTOP is a fast, terminal-based system monitor written in Rust, inspired by `top`.
It shows CPU and memory usage with live history graphs, plus a navigable process
list with the ability to terminate processes.

</div>

## Features

- **Live CPU monitoring** — total usage, per-core usage, and a scrolling history graph.
- **Live memory monitoring** — used, available, free, and total memory, each with its own history graph.
- **Process list** — PID, name (with executable path), user, thread count, CPU %, and memory usage.
- **Kill processes** — with a confirmation prompt and an error toast if the operation fails.
- **Vim-style keybindings** — `j`/`k`/`g`/`G` for navigation, `h`/`?` for help, `q` to quit.
- **Responsive layout** — per-core CPU stats are laid out in multiple columns based on terminal width.
- **Non-blocking sampling** — system data is collected on background threads, so the UI stays responsive.

## Requirements

- Rust **1.95+** (the project uses the 2024 edition).
- Linux, macOS, Windows, or another platform supported by [`sysinfo`](https://crates.io/crates/sysinfo).
- A terminal with UTF-8 and color support.

## Installation

### From source

```bash
git clone https://github.com/Sckab/rtop.git
cd rtop
cargo build --release
./target/release/rtop
```

To install the binary into `~/.cargo/bin`:

```bash
cargo install --path .
```

Or install directly from the repository:

```bash
cargo install --git https://github.com/Sckab/rtop.git
```

## Usage

```bash
rtop
```

RTOP takes no command-line arguments (other than `--help` and `--version`) and
has no configuration file: it starts in the alternate screen and exits with `q`
or `Esc`.

### Keybindings

| Key        | Action                            |
| ---------- | --------------------------------- |
| `h`, `?`   | Show / hide the help panel        |
| `q`, `Esc` | Quit (or close the current panel) |
| `k`, `↑`   | Select the previous process       |
| `j`, `↓`   | Select the next process           |
| `g`, `←`   | Select the first process          |
| `G`, `→`   | Select the last process           |
| `K`        | Kill the selected process         |
| `/`        | Filter processes                  |
| `Esc`      | Cancel the filter input           |
| `Enter`    | Confirm the filter                |

Inside the kill confirmation dialog:

| Key                  | Action           |
| -------------------- | ---------------- |
| `y`, `Enter`         | Confirm the kill |
| `n`, `N`, `q`, `Esc` | Cancel           |

Navigation wraps around: pressing `j` on the last process jumps to the first one,
and `k` on the first process jumps to the last one.

## Interface

The screen is divided into three areas:

1. **Header** — `user@host | OS` on the left, current local time on the right.
2. **CPU and memory panels** — side by side, each with live usage values and a
   braille history graph.
3. **Process table** — all visible processes with the currently selected one
   highlighted. The panel title shows the selected position and the total number
   of processes, e.g. `Processes (3/214)`.

Additional overlays:

- **Help panel** (`h` / `?`) — lists all keybindings; navigable with the same
  movement keys.
- **Kill confirmation** (`K`) — asks for confirmation before sending the signal.
- **Error toast** — appears in the top-right corner for 3 seconds if a process
  cannot be killed (e.g. insufficient permissions).
- **Filter input** (`/`) — opens an input field to filter processes by name,
  executable path. Updates the process list as you type; press `Esc` to clear
  the filter.

> Note: killing a process requires the appropriate permissions. On Unix systems,
> the process is terminated with `SIGKILL`, and processes owned by other users can
> only be killed by root.

## How It Works

RTOP uses three threads:

- **UI thread** — renders with [`ratatui`](https://crates.io/crates/ratatui) and
  [`crossterm`](https://crates.io/crates/crossterm), polls terminal events every
  50 ms, and redraws only when something changed.
- **Stats thread** — samples CPU and memory via [`sysinfo`](https://crates.io/crates/sysinfo)
  at `sysinfo::MINIMUM_CPU_UPDATE_INTERVAL`.
- **Process thread** — refreshes the process list every 500 ms and handles kill
  requests sent by the UI.

The threads communicate through `std::sync::mpsc` channels, so sampling never
blocks input handling or rendering. Display strings are precomputed once per
refresh rather than on every frame.

## Development

```bash
cargo run              # Run in debug mode
cargo build --release  # Optimized build
cargo fmt              # Format code
cargo clippy           # Run lints
```

## Limitations

- No sorting of the process list yet.
- No configurable refresh intervals or themes.
- No tree visualization of the processes.
- Process names and executable paths for processes owned by other users may be
  unavailable on some platforms.
- Killing processes is the only write operation performed by the application.

## License

Distributed under the terms of the **GNU General Public License v3.0**.
See [LICENSE](LICENSE) for the full text.
