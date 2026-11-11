use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Row, StatefulWidget, Table},
};

use crate::{
    app::state::Mode,
    system::process::{Process, Sort, SortType},
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

fn sort_processes(processes: &mut Vec<&Process>, sort: &Sort) {
    match sort {
        Sort::Pid(sort_type) => match sort_type {
            SortType::Ascending => {
                processes.sort_by_key(|p| p.pid);
            }
            SortType::Descending => {
                processes.sort_by_key(|p| std::cmp::Reverse(p.pid));
            }
        },
        Sort::Name(sort_type) => match sort_type {
            SortType::Ascending => {
                processes.sort_by_key(|p| p.name.clone());
            }
            SortType::Descending => {
                processes.sort_by_key(|p| std::cmp::Reverse(p.name.clone()));
            }
        },
        Sort::Exe(sort_type) => match sort_type {
            SortType::Ascending => {
                processes.sort_by_key(|p| p.exe.clone());
            }
            SortType::Descending => {
                processes.sort_by_key(|p| std::cmp::Reverse(p.exe.clone()));
            }
        },
        Sort::User(sort_type) => match sort_type {
            SortType::Ascending => {
                processes.sort_by_key(|p| p.user.clone());
            }
            SortType::Descending => {
                processes.sort_by_key(|p| std::cmp::Reverse(p.user.clone()));
            }
        },
        Sort::Thread(sort_type) => match sort_type {
            SortType::Ascending => {
                processes.sort_by_key(|p| p.threads);
            }
            SortType::Descending => {
                processes.sort_by_key(|p| std::cmp::Reverse(p.threads));
            }
        },
        Sort::Cpu(sort_type) => match sort_type {
            SortType::Ascending => {
                processes.sort_by(|a, b| a.cpu_usage.total_cmp(&b.cpu_usage));
            }
            SortType::Descending => {
                processes.sort_by(|a, b| b.cpu_usage.total_cmp(&a.cpu_usage));
            }
        },
        Sort::Memory(sort_type) => match sort_type {
            SortType::Ascending => {
                processes.sort_by_key(|p| p.memory);
            }
            SortType::Descending => {
                processes.sort_by_key(|p| std::cmp::Reverse(p.memory));
            }
        },
        Sort::None => {}
    }
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
        let query = self.current_query();

        let mut filtered_processes =
            Self::filtered_processes(query, &self.processes_state.processes);

        sort_processes(&mut filtered_processes, &self.processes_state.sort);

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
            && !matches!(self.mode, Mode::Search { .. })
        {
            format!("{title} Search: {query}")
        } else {
            title
        };

        let area =
            super::widgets::get_bordered_block(format!(" {title} "), false, false, area, buf);

        let mut table_columns = [
            "PID".to_string(),
            "Name".to_string(),
            "User".to_string(),
            "Threads".to_string(),
            "CPU %".to_string(),
            "Memory".to_string(),
        ];

        match &self.processes_state.sort {
            Sort::Pid(sort_type) => table_columns[0].push_str(match sort_type {
                SortType::Ascending => " ",
                SortType::Descending => " ",
            }),
            Sort::Name(sort_type) => table_columns[1].push_str(match sort_type {
                SortType::Ascending => " ",
                SortType::Descending => " ",
            }),
            Sort::Exe(sort_type) => table_columns[1].push_str(match sort_type {
                SortType::Ascending => "  (exe)",
                SortType::Descending => "  (exe)",
            }),
            Sort::User(sort_type) => table_columns[2].push_str(match sort_type {
                SortType::Ascending => " ",
                SortType::Descending => " ",
            }),
            Sort::Thread(sort_type) => table_columns[3].push_str(match sort_type {
                SortType::Ascending => " ",
                SortType::Descending => " ",
            }),
            Sort::Cpu(sort_type) => table_columns[4].push_str(match sort_type {
                SortType::Ascending => " ",
                SortType::Descending => " ",
            }),
            Sort::Memory(sort_type) => table_columns[5].push_str(match sort_type {
                SortType::Ascending => " ",
                SortType::Descending => " ",
            }),
            Sort::None => {}
        }

        let header = Row::new(table_columns.clone()).fg(Color::Green);

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
                    Constraint::Length(
                        if matches!(
                            self.processes_state.sort,
                            Sort::Cpu(SortType::Ascending) | Sort::Cpu(SortType::Descending)
                        ) {
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
            .row_highlight_style(Style::new().bg(Color::Green).fg(Color::Black)),
            area,
            buf,
            &mut self.tables.process_table,
        );
    }
}
