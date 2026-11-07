use std::{ffi::OsString, path::PathBuf};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Row, StatefulWidget, Table},
};

#[derive(Debug, Default, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: OsString,
    pub exe: PathBuf,
    pub user: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub threads: usize,
}

impl super::App {
    pub fn render_processes(&mut self, area: Rect, buf: &mut Buffer) {
        let selected = match self.states.process_table.selected() {
            Some(index) if !self.processes.is_empty() => index.min(self.processes.len() - 1) + 1,
            _ => 0,
        };

        let area = super::general::get_borderd_block(
            format!(" Processes ({selected}/{}) ", self.processes.len()),
            area,
            buf,
        );

        let format_name = |p: &Process| -> String {
            let process_exe = p.exe.display();

            let l_par = if process_exe.to_string().is_empty() {
                ""
            } else {
                "("
            };

            let r_par = if process_exe.to_string().is_empty() {
                ""
            } else {
                ")"
            };

            format!("{} {}{}{}", p.name.display(), l_par, process_exe, r_par)
        };

        let header =
            Row::new(["PID", "Name", "User", "Threads", "CPU %", "Memory"]).fg(Color::Green);

        let mut pid_width = "PID".len();
        let mut name_width = "Name".len();
        let mut user_width = "User".len();
        let mut threads_width = "Threads".len();
        let mut memory_width = "Memory".len();

        for process in &self.processes {
            pid_width = pid_width.max(process.pid.to_string().len());
            name_width = name_width.max(format_name(process).len());
            user_width = user_width.max(process.user.len());
            threads_width = threads_width.max(process.threads.to_string().len());
            memory_width =
                memory_width.max((self.system_info.size_formatter)(process.memory).len());
        }

        let rows: Vec<Row> = self
            .processes
            .iter()
            .map(|p| {
                Row::new([
                    p.pid.to_string(),
                    format_name(p),
                    p.user.clone(),
                    p.threads.to_string(),
                    format!("{:.1}%", p.cpu_usage),
                    (self.system_info.size_formatter)(p.memory),
                ])
            })
            .collect();

        Table::new(
            rows,
            [
                Constraint::Length(pid_width as u16),
                Constraint::Length(name_width as u16),
                Constraint::Length(user_width as u16),
                Constraint::Length(threads_width as u16),
                Constraint::Length(6),
                Constraint::Length(memory_width as u16),
            ],
        )
        .column_spacing(2)
        .header(header)
        .row_highlight_style(Style::new().bg(Color::Green).fg(Color::Black))
        .render(area, buf, &mut self.states.process_table);
    }
}
