use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Clear, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
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

impl super::App {
    pub fn render_processes(&mut self, area: Rect, buf: &mut Buffer) {
        let selected_index = self.states.process_table.selected();

        let selected = match selected_index {
            Some(index) if !self.processes.is_empty() => index.min(self.processes.len() - 1) + 1,
            _ => 0,
        };

        self.states.selected_process_pid = selected_index
            .and_then(|index| self.processes.get(index))
            .map(|process| process.pid)
            .unwrap_or_default();

        self.states.selected_process_name = selected_index
            .and_then(|index| self.processes.get(index))
            .map(|process| process.name.clone())
            .unwrap_or_default();

        let area = super::general::get_borderd_block(
            format!(" Processes ({selected}/{}) ", self.processes.len()),
            false,
            false,
            area,
            buf,
        );

        let header =
            Row::new(["PID", "Name", "User", "Threads", "CPU %", "Memory"]).fg(Color::Green);

        let visible_rows = area.height.saturating_sub(1) as usize;

        let offset = Self::process_table_offset(
            selected_index.unwrap_or(0),
            self.states.process_table.offset(),
            visible_rows,
            self.processes.len(),
        );
        *self.states.process_table.offset_mut() = offset;

        let visible_processes = || self.processes.iter().skip(offset).take(visible_rows);

        let mut pid_width = "PID".len();
        let mut name_width = "Name".len();
        let mut user_width = "User".len();
        let mut threads_width = "Threads".len();
        let mut cpu_width = "CPU %".len();
        let mut memory_width = "Memory".len();

        for process in &self.processes {
            pid_width = pid_width.max(process.pid_display.len());
            name_width = name_width.max(process.display_name.len());
            user_width = user_width.max(process.user.len());
            threads_width = threads_width.max(process.threads_display.len());
            cpu_width = cpu_width.max(process.cpu_display.len());
            memory_width = memory_width.max(process.memory_display.len());
        }

        let rows: Vec<Row> = visible_processes()
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

        let mut table_state =
            TableState::default()
                .with_offset(0)
                .with_selected(selected_index.map(|index| {
                    index
                        .min(self.processes.len().saturating_sub(1))
                        .saturating_sub(offset)
                }));

        StatefulWidget::render(
            Table::new(
                rows,
                [
                    Constraint::Length(pid_width as u16),
                    Constraint::Length(name_width as u16),
                    Constraint::Length(user_width as u16),
                    Constraint::Length(threads_width as u16),
                    Constraint::Length(cpu_width as u16),
                    Constraint::Length(memory_width as u16),
                ],
            )
            .column_spacing(2)
            .header(header)
            .row_highlight_style(Style::new().bg(Color::Green).fg(Color::Black)),
            area,
            buf,
            &mut table_state,
        );
    }

    fn process_table_offset(selected: usize, current: usize, visible: usize, len: usize) -> usize {
        if len == 0 || visible == 0 {
            return 0;
        }

        let selected = selected.min(len - 1);
        let max_offset = len.saturating_sub(visible);

        if selected < current {
            selected
        } else if selected >= current + visible {
            (selected + 1 - visible).min(max_offset)
        } else {
            current.min(max_offset)
        }
    }

    pub fn render_kill_confirmation(&self, area: Rect, buf: &mut Buffer) {
        let main_text = format!(
            "Are you sure to kill \"{}\"?",
            self.states.selected_process_name.display()
        );

        let area = area.centered(
            Constraint::Length((main_text.cell_width() + 2) as u16),
            Constraint::Length(5),
        );

        ratatui::widgets::Widget::render(Clear, area, buf);

        let popup_area = super::general::get_borderd_block(" Confirm ", false, true, area, buf);

        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(popup_area);

        Paragraph::new(main_text).render(layout[0], buf);
        Paragraph::new(format!("PID: {}", self.states.selected_process_pid)).render(layout[1], buf);
        Paragraph::new(
            Span::raw("[Y] Yes").fg(Color::Red)
                + Span::raw(" ".repeat(4))
                + Span::raw("[N] No").fg(Color::Green),
        )
        .alignment(Alignment::Center)
        .render(layout[2], buf);
    }

    pub fn render_kill_failed(&self, area: Rect, buf: &mut Buffer) {
        let area = area.centered(
            Constraint::Length(self.failed_message.cell_width() + 2),
            Constraint::Length(3),
        );

        ratatui::widgets::Widget::render(Clear, area, buf);

        let popup_area = super::general::get_borderd_block(" Error ", true, true, area, buf);

        Paragraph::new(self.failed_message.clone()).render(popup_area, buf);
    }
}
