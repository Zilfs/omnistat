use std::{io, sync::mpsc::Receiver, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::core::memory::MemoryStats;

pub mod core;
pub mod ui;
pub mod utils;

#[derive(Debug, Default)]
pub struct App {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub exit: bool,
}

impl App {
    fn draw(&self, frame: &mut Frame) {
        ui::render(self, frame.area(), frame.buffer_mut());
    }

    pub fn run(
        &mut self,
        mut terminal: DefaultTerminal,
        rx: Receiver<MemoryStats>,
    ) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if let Ok(stats) = rx.try_recv() {
                self.total_memory = stats.total_memory;
                self.used_memory = stats.used_memory;
                self.free_memory = stats.free_memory;
            }

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                        && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
                    {
                        self.exit = true
                    }
                }
            }
        }
        Ok(())
    }
}
