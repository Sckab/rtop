<div align="center">

# RTOP

_A Rust TOP_

[![Rust](https://img.shields.io/badge/rust-1.95%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue)](LICENSE)

![showcase](./screenshots/showcase.png)

RTOP is a fast, terminal-based system monitor written in Rust, inspired by `top`.
It shows CPU and memory usage with live history graphs, live hardware
temperatures, and sortable process and temperature tables; the process list can
be filtered and processes can be terminated.

</div>

## Features

- **Live CPU monitoring** — CPU brand and average frequency, load average (1, 5, and 15 minutes; Unix only), total and per-core usage, and a scrolling history graph.
- **Live memory monitoring** — used, available, and free memory in a combined scrolling history graph; total memory is shown as a value. On Windows, available and free are identical, so only free is listed.
- **Temperature monitoring** — lists hardware sensors with their current temperature in a selectable, sortable table, refreshed every second. The selected sensor is preserved across refreshes and sorting. Available sensors depend on the platform and drivers; unavailable readings are shown as `Unknown`.
- **Process list** — PID, name (with executable path), user, thread count, CPU %, and memory usage.
- **Sorting and filtering** — sort the process and temperature tables with `Alt`-based shortcuts, hold `Shift` for descending, and filter processes by name or executable path. The active sort column is marked with an arrow.
- **Kill processes** — with a confirmation prompt, a start-time check to avoid killing a recycled PID, and an error toast if the operation fails.
- **Vim-style keybindings** — `j`/`k`/`g`/`G` for navigation in the focused panel, `H`/`L` to switch panels, `h`/`?` for help, `q` to quit, plus `Alt`-based sorting.
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

| Key        | Action                                       |
| ---------- | -------------------------------------------- |
| `H`        | Focus the Temperatures panel (left)          |
| `L`        | Focus the Processes panel (right)            |
| `h`, `?`   | Show / hide the help panel                   |
| `q`, `Esc` | Quit (or close the current panel)            |
| `k`, `↑`   | Select the previous row in the focused panel |
| `j`, `↓`   | Select the next row in the focused panel     |
| `g`, `←`   | Select the first row in the focused panel    |
| `G`, `→`   | Select the last row in the focused panel     |
| `K`        | Kill the selected process                    |
| `/`        | Filter processes                             |
| `Esc`      | Cancel the filter input                      |
| `Enter`    | Confirm the filter                           |

Navigation wraps within the focused panel: pressing `j` on the last row jumps to
the first one, and `k` on the first row jumps to the last one. `K` and `/`
always act on the process list, even while the Temperatures panel is focused;
filtering is available for processes only.

Process sorting (while the Processes panel is focused):

| Key       | Action                  |
| --------- | ----------------------- |
| `Alt + p` | Sort by PID             |
| `Alt + n` | Sort by process name    |
| `Alt + e` | Sort by executable path |
| `Alt + u` | Sort by user            |
| `Alt + t` | Sort by thread count    |
| `Alt + c` | Sort by CPU usage       |
| `Alt + m` | Sort by memory usage    |

Temperature sorting (while the Temperatures panel is focused):

| Key       | Action              |
| --------- | ------------------- |
| `Alt + s` | Sort by sensor      |
| `Alt + t` | Sort by temperature |

Selecting a column sorts in ascending order by default; hold `Shift` while
pressing the shortcut (e.g. `Shift + Alt + c`) to sort in descending order.
`Alt + t` is contextual: it sorts processes by thread count and temperatures by
temperature value. `Alt + r` resets sorting for the focused panel.

Process sorting is applied to the filtered process list, and the active column
is marked with an arrow in the process table header (`(exe)` is shown when
sorting by executable path). Temperature sorting is applied immediately and is
marked with an arrow in the temperature table header. The selected sensor stays
selected across refreshes and sorting, and temperatures are sorted by sensor name
in ascending order by default.

Inside the kill confirmation dialog:

| Key                  | Action           |
| -------------------- | ---------------- |
| `y`, `Enter`         | Confirm the kill |
| `n`, `N`, `q`, `Esc` | Cancel           |

## Interface

The screen is divided into four areas:

1. **Header** — `user@host | OS` on the left, current local time on the right.
2. **CPU and memory panels** — side by side at the top. The CPU panel shows the
   CPU brand and average frequency on the left, the load average (1, 5, and
   15 minutes) on the right, the per-core usage grid with total CPU usage, and a
   braille history graph. The memory panel shows used, available, and free memory
   in a combined braille history graph, plus total memory as a value.
3. **Temperatures panel** — below the CPU panel. It lists the available hardware
   sensors with their current temperature in a `Sensor` / `Temperature` table,
   sorted by sensor name in ascending order by default. The panel title shows the
   selected position and the total number of sensors, e.g. `Temperatures (2/8)`,
   and the panel width adapts to its content. Readings are refreshed every
   second, and sensors without a readable value are shown as `Unknown`. The
   selected row is highlighted only while the panel is focused.
4. **Process table** — to the right of the temperatures panel. It lists all
   visible processes with the currently selected one highlighted while the panel
   is focused. The panel title shows the selected position and the total number
   of processes, e.g. `Processes (3/214)`, and appends `Search: <query>` while a
   filter is active. The header marks the active sort column with an arrow.

Only the focused panel receives navigation and sorting shortcuts. Use `H` to
focus the Temperatures panel (left) and `L` to focus the Processes panel
(right); the selected row is highlighted in the focused panel.

> On Windows, available memory is always equal to free memory, so the Available
> row and its graph line are omitted; the panel shows Used, Free, and Total.
>
> On Windows, the load average is not available and is omitted from the CPU panel.

Additional overlays:

- **Help panel** (`h` / `?`) — static panel with **General** keys and **Sorting**
  shortcuts, split into **Processes** and **Temperatures** sections; close it
  with `h`, `?`, `q`, or `Esc`.
- **Kill confirmation** (`K`) — asks for confirmation before sending the signal.
- **Error toast** — appears in the top-right corner for 3 seconds if a process
  cannot be killed (e.g. insufficient permissions).
- **Filter input** (`/`) — filters processes by name or executable path and
  updates the list as you type. Press `Enter` to keep the filter or `Esc` to
  clear it.

> Note: killing a process requires the appropriate permissions. On Unix systems,
> the process is terminated with `SIGKILL`, and processes owned by other users can
> only be killed by root. RTOP also checks the process start time before sending
> the signal, so a recycled PID is not killed by mistake.

## How It Works

RTOP uses four threads:

- **UI thread** — renders with [`ratatui`](https://crates.io/crates/ratatui) and
  [`crossterm`](https://crates.io/crates/crossterm), polls terminal events every
  100 ms, and redraws only when something changed.
- **Stats thread** — samples CPU usage, average frequency, load average, and
  memory via [`sysinfo`](https://crates.io/crates/sysinfo) every 500 ms, keeps a
  bounded history, and renders only the samples that fit in the graph, so
  scrolling starts as soon as the visible window fills.
- **Process thread** — refreshes the process list every 500 ms and handles kill
  requests sent by the UI.
- **Temperature thread** — refreshes hardware sensors via
  [`sysinfo`](https://crates.io/crates/sysinfo) every second and sends the
  readings to the UI.

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

- No configurable refresh intervals or themes.
- Sorting and filter state are not persisted between runs.
- No tree visualization of the processes.
- Filtering is available for processes only.
- Process names and executable paths for processes owned by other users may be
  unavailable on some platforms.
- Temperature sensors depend on the hardware and drivers; on systems where none
  are exposed, the Temperatures panel only shows its header.
- Killing processes is the only write operation performed by the application.

## License

Distributed under the terms of the **GNU General Public License v3.0**.
See [LICENSE](LICENSE) for the full text.
