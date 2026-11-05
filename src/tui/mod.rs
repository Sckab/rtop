use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Paragraph, Widget},
};
use std::io;

mod cpu;
mod memory;

#[derive(Default, Debug)]
pub struct Memory {
    available: u64,
    used: u64,
    free: u64,
    total: u64,
}

pub struct App {
    quit: bool,
    system: sysinfo::System,
    host_name: String,
    os_name: String,
    user_name: String,
    cpu_usage: f32,
    memory: Memory,
    size_formatter: Box<dyn Fn(u64) -> String>,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            quit: false,
            system: sysinfo::System::new()?,
            host_name: sysinfo::System::host_name()?,
            os_name: sysinfo::System::name()?,
            cpu_usage: 0.0,
            memory: Memory::default(),
            size_formatter: Box::new(humansize::make_format(humansize::DECIMAL)),

            #[cfg(unix)]
            user_name: std::env::var("USER").unwrap_or("unkown".to_string()),

            #[cfg(windows)]
            user_name: std::env::var("USERNAME").unwrap_or("unkown".to_string()),
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.quit {
            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

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

        Paragraph::new(format!(
            "{}@{} | {}",
            self.user_name, self.host_name, self.os_name
        ))
        .render(layout[0], buf);

        self.render_cpu_usage(info_layout[0], buf);
        self.render_memory_usage(info_layout[1], buf);
    }
}
