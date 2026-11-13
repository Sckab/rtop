use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    text::Span,
    widgets::{Cell, Row, StatefulWidget, Table},
};

use crate::{
    app::state::{Mode, SelectedPanel},
    system::{
        process::Process,
        sorting::{ProcessSort, ProcessSortType},
    },
    tui::widgets::get_selected_style,
};

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

    pub fn filtered_and_sorted_processes<'a>(
        query: Option<&str>,
        processes: &'a [Process],
        sort: &ProcessSort,
    ) -> Vec<&'a Process> {
        let mut filtered = Self::filtered_processes(query, processes);

        sort.sort_processes(&mut filtered);

        filtered
    }

    pub fn render_processes(&mut self, area: Rect, buf: &mut Buffer) {
        let query = self.current_query();
        let sort = &self.processes_state.sort;

        let filtered_processes =
            Self::filtered_and_sorted_processes(query, &self.processes_state.processes, sort);

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
            && !matches!(self.state.mode, Mode::Search { .. })
        {
            format!("{title} Search: {query}")
        } else {
            title
        };

        let area =
            super::widgets::get_bordered_block(format!(" {title} "), false, false, area, buf);

        let table_columns = [
            format!("PID{}", sort.get_arrow(ProcessSortType::Pid)),
            format!("Name{}", sort.get_arrow(ProcessSortType::Name)),
            format!("User{}", sort.get_arrow(ProcessSortType::User)),
            format!("Threads{}", sort.get_arrow(ProcessSortType::Thread)),
            format!("CPU %{}", sort.get_arrow(ProcessSortType::Cpu)),
            format!("Memory{}", sort.get_arrow(ProcessSortType::Memory)),
        ];

        let header = Row::new([
            Cell::new(Span::raw(table_columns[0].clone()).into_right_aligned_line()),
            Cell::new(table_columns[1].clone()),
            Cell::new(table_columns[2].clone()),
            Cell::new(table_columns[3].clone()),
            Cell::new(table_columns[4].clone()),
            Cell::new(table_columns[5].clone()),
        ])
        .fg(Color::Green);

        let mut pid_width = table_columns[0].len();
        let mut name_width = table_columns[1].len();
        let mut user_width = table_columns[2].len();
        let mut threads_width = table_columns[3].len();
        let mut memory_width = table_columns[4].len();

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
                let pid = Span::raw(p.pid_display.as_str()).into_right_aligned_line();

                Row::new([
                    Cell::new(pid),
                    Cell::new(p.display_name.as_str()),
                    Cell::new(p.user.as_str()),
                    Cell::new(p.threads_display.as_str()),
                    Cell::new(p.cpu_display.as_str()),
                    Cell::new(p.memory_display.as_str()),
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
                    Constraint::Length(
                        if matches!(self.processes_state.sort.sort_type, ProcessSortType::Cpu) {
                            7
                        } else {
                            6
                        },
                    ),
                    Constraint::Length(memory_width as u16),
                ],
            )
            .column_spacing(2)
            .header(header)
            .row_highlight_style(get_selected_style(matches!(
                &self.state.selected_panel,
                SelectedPanel::Processes
            ))),
            area,
            buf,
            &mut self.tables.process_table,
        );
    }
}
