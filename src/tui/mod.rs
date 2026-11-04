use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Paragraph, Widget},
};
use std::io;

#[derive(Debug)]
pub struct App {
    quit: bool,
    system: sysinfo::System,
    host_name: String,
    name: String,
    cpu_usage: f32,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let system = sysinfo::System::new()?;

        Ok(Self {
            quit: false,
            system,
            host_name: sysinfo::System::host_name()?,
            name: sysinfo::System::name()?,
            cpu_usage: 0.0,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.quit {
            self.system.refresh_all();

            self.cpu_usage = self.system.global_cpu_usage();

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
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .split(area);

        Paragraph::new(format!("{} | {}", self.host_name, self.name)).render(layout[0], buf);
        Paragraph::new(format!("CPU usage: {}", self.cpu_usage))
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(layout[1], buf);
    }
}
