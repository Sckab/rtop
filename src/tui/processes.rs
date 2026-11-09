use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Row, StatefulWidget, Table},
};

use crate::system::process::Process;

pub fn prepare_display(process: &mut Process, size_formatter: &dyn Fn(u64) -> String) {
    let exe = process.exe.display().to_string();

    process.display_name = if exe.is_empty() {
        process.name.display().to_string()
    } else {
        format!("{} ({})", process.name.display(), exe)
    };
    process.pid_display = process.pid.to_string();
    process.threads_display = process.threads.to_string();
    process.cpu_display = format!("{:.1}%", process.cpu_usage);
    process.memory_display = size_formatter(process.memory);
}

impl crate::app::App {
    pub fn filtered_processes<'a>(
        query: Option<&str>,
        processes: &'a [Process],
    ) -> Vec<&'a Process> {
        match query {
            Some(query) => processes
                .iter()
                .filter(|p| {
                    p.name.to_string_lossy().contains(query)
                        || p.exe.to_string_lossy().contains(query)
                })
                .collect(),
            None => processes.iter().collect(),
        }
    }

    pub fn render_processes(&mut self, area: Rect, buf: &mut Buffer) {
        let query = match &self.mode {
            crate::app::state::Mode::Search { query } => Some(query.as_str()),
            _ => self.process_query.as_deref(),
        };

        let filtered_processes = Self::filtered_processes(query, &self.processes);

        let selected_index = self.tables.process_table.selected();

        let selected = match selected_index {
            Some(index) if !filtered_processes.is_empty() => {
                index.min(filtered_processes.len() - 1) + 1
            }
            _ => 0,
        };

        let title = format!("Processes ({selected}/{})", filtered_processes.len());

        let title = if let Some(query) = query
            && !query.is_empty()
        {
            format!("{title} Search: {query}")
        } else {
            title
        };

        let area =
            super::widgets::get_bordered_block(format!(" {title} "), false, false, area, buf);

        let header =
            Row::new(["PID", "Name", "User", "Threads", "CPU %", "Memory"]).fg(Color::Green);

        let mut pid_width = "PID".len();
        let mut name_width = "Name".len();
        let mut user_width = "User".len();
        let mut threads_width = "Threads".len();
        let mut memory_width = "Memory".len();

        for process in &filtered_processes {
            pid_width = pid_width.max(process.pid_display.len());
            name_width = name_width.max(process.display_name.len());
            user_width = user_width.max(process.user.len());
            threads_width = threads_width.max(process.threads_display.len());
            memory_width = memory_width.max(process.memory_display.len());
        }

        let rows: Vec<Row> = filtered_processes
            .iter()
            .map(|p| {
                Row::new([
                    p.pid_display.as_str(),
                    p.display_name.as_str(),
                    p.user.as_str(),
                    p.threads_display.as_str(),
                    p.cpu_display.as_str(),
                    p.memory_display.as_str(),
                ])
            })
            .collect();

        StatefulWidget::render(
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
            .row_highlight_style(Style::new().bg(Color::Green).fg(Color::Black)),
            area,
            buf,
            &mut self.tables.process_table,
        );
    }
}
