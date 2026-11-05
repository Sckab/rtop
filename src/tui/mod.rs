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

#[derive(Debug)]
pub struct CpuStats {
    pub number: usize,
    pub usage: f32,
}

#[derive(Debug, Default)]
pub struct SystemStats {
    pub global_cpu_usage: f32,
    pub memory_used: u64,
    pub memory_available: u64,
    pub memory_free: u64,
    pub memory_total: u64,
    pub cpus: Vec<CpuStats>,
}

pub struct SystemInfo {
    host_name: String,
    os_name: String,
    user_name: String,
    size_formatter: Box<dyn Fn(u64) -> String>,
}

pub struct App {
    quit: bool,
    system_info: SystemInfo,
    stats: SystemStats,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let system_info = SystemInfo {
            host_name: sysinfo::System::host_name()?,
            os_name: sysinfo::System::name()?,
            size_formatter: Box::new(humansize::make_format(humansize::DECIMAL)),

            #[cfg(unix)]
            user_name: std::env::var("USER").unwrap_or("unkown".to_string()),

            #[cfg(windows)]
            user_name: std::env::var("USERNAME").unwrap_or("unkown".to_string()),
        };

        Ok(Self {
            quit: false,
            system_info,
            stats: SystemStats::default(),
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel::<SystemStats>();

        // we need to create a separate thread as the std::thread::sleep() will block the user input events
        std::thread::spawn(move || {
            let mut system = match sysinfo::System::new() {
                Ok(system) => system,
                Err(error) => {
                    eprintln!("Failed to initialize system: {error}");
                    return;
                }
            };

            loop {
                system.refresh_all();

                let mut cpus: Vec<CpuStats> = vec![];

                for (mut index, cpu) in system.cpus().iter().enumerate() {
                    index += 1;

                    cpus.push(CpuStats {
                        number: index,
                        usage: cpu.usage(),
                    });
                }

                let stats = SystemStats {
                    global_cpu_usage: system.global_cpu_usage(),
                    memory_used: system.used_memory(),
                    memory_available: system.available_memory(),
                    memory_free: system.free_memory(),
                    memory_total: system.total_memory(),
                    cpus,
                };

                if tx.send(stats).is_err() {
                    break;
                }

                std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            }
        });

        while !self.quit {
            while let Ok(stats) = rx.try_recv() {
                self.stats = stats;
            }

            self.handle_events()?;
            terminal.draw(|frame| self.draw(frame))?;
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
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
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
            self.system_info.user_name, self.system_info.host_name, self.system_info.os_name
        ))
        .render(layout[0], buf);

        self.render_cpu_usage(info_layout[0], buf);
        self.render_memory_usage(info_layout[1], buf);
    }
}
