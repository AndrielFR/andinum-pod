use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode};
use systemstat::Platform;

use crate::ui::InfoStats;

pub struct App {
    pub version: String,

    pub tabs: TabsState,
    pub info_stats: InfoStats,
}

impl App {
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),

            tabs: TabsState::new(vec!["Info", "Queue", "Settings"]),
            info_stats: InfoStats::new(systemstat::System::new()),
        }
    }

    pub fn on_up(&mut self) {}

    pub fn on_down(&mut self) {}

    pub fn on_left(&mut self, can_change_tab: bool) {
        if can_change_tab {
            self.tabs.previous();
        }
    }

    pub fn on_right(&mut self, can_change_tab: bool) {
        if can_change_tab {
            self.tabs.next();
        }
    }

    pub fn on_key(&mut self, _key: char) {}

    pub fn handle_events(&mut self) -> std::io::Result<bool> {
        if event::poll(Duration::from_millis(50)).is_ok() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == event::KeyEventKind::Press {
                    use KeyCode::*;

                    match key.code {
                        Up | Char('k') => self.on_up(),
                        Down | Char('j') => self.on_down(),
                        Left => self.on_left(true),
                        Char('h') => self.on_left(false),
                        Right => self.on_right(true),
                        Char('l') => self.on_right(false),
                        Tab => self.on_right(true),
                        Char('q') => return Ok(true),
                        Char(c) => self.on_key(c),
                        _ => {}
                    }
                }
            }
        }

        Ok(false)
    }
}

pub struct TabsState {
    pub titles: Vec<&'static str>,
    pub index: usize,
}

impl TabsState {
    pub fn new(titles: Vec<&'static str>) -> Self {
        Self { titles, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
