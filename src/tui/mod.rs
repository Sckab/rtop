use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Paragraph, Row, Table, Widget},
};
use std::io;

#[derive(Default, Debug)]
pub struct Memory {
    available: u64,
    used: u64,
    free: u64,
    total: u64,
}

#[derive(Debug)]
pub struct App {
    quit: bool,
    system: sysinfo::System,
    host_name: String,
    name: String,
    cpu_usage: f32,
    memory: Memory,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            quit: false,
            system: sysinfo::System::new()?,
            host_name: sysinfo::System::host_name()?,
            name: sysinfo::System::name()?,
            cpu_usage: 0.0,
            memory: Memory::default(),
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.quit {
            self.system.refresh_all();

            self.cpu_usage = self.system.global_cpu_usage();
            self.memory.used = self.system.used_memory();
            self.memory.available = self.system.available_memory();
            self.memory.free = self.system.free_memory();
            self.memory.total = self.system.total_memory();

            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.quit = true,
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .split(area);

        let info_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        Paragraph::new(format!("{} | {}", self.host_name, self.name)).render(layout[0], buf);
        Paragraph::new(format!("CPU usage: {}", self.cpu_usage))
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(info_layout[0], buf);

        let table_header = Row::new(["Memory", "Value"]);

        let rows = [
            Row::new(["Used".to_string(), self.memory.used.to_string()]),
            Row::new(["Available".to_string(), self.memory.available.to_string()]),
            Row::new(["Free".to_string(), self.memory.free.to_string()]),
            Row::new(["Total".to_string(), self.memory.total.to_string()]),
        ];

        let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];

        let table = Table::new(rows, widths)
            .header(table_header)
            .column_spacing(1)
            .block(Block::bordered().border_type(BorderType::Rounded));

        table.render(info_layout[1], buf);
    }
}
