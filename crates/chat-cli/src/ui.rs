use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::event::{AppEvent, EventHandler};

mod chats;
mod home;
mod nav;

#[derive(Debug)]
pub enum NavSection {
    Home(home::Home),
    Chats(chats::Chats),
    Settings,
    Help,
}

impl Default for NavSection {
    fn default() -> Self {
        Self::Home(Default::default())
    }
}

impl NavSection {
    pub fn index(&self) -> usize {
        match self {
            NavSection::Home(_) => 0,
            NavSection::Chats(_) => 1,
            NavSection::Settings => 2,
            NavSection::Help => 3,
        }
    }

    pub fn prev(&self) -> NavSection {
        match self {
            NavSection::Home(_) => Default::default(),
            NavSection::Chats(_) => NavSection::Home(Default::default()),
            NavSection::Settings => NavSection::Chats(Default::default()),
            NavSection::Help => NavSection::Settings,
        }
    }

    pub fn next(&self) -> NavSection {
        match self {
            NavSection::Home(_) => NavSection::Chats(Default::default()),
            NavSection::Chats(_) => NavSection::Settings,
            NavSection::Settings => NavSection::Help,
            NavSection::Help => NavSection::Help,
        }
    }
}

impl Widget for &NavSection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            NavSection::Home(section) => section.render(area, buf),
            NavSection::Chats(section) => section.render(area, buf),
            NavSection::Settings => {},
            NavSection::Help => {}
        }
    }
}

pub trait UIKeyHandler {
    fn handle(&mut self, events: &mut EventHandler, key_event: KeyEvent);
}

#[derive(Debug, Default)]
pub struct State {
    nav_bar: nav::NavBar,
    section: NavSection,
}

impl Widget for &State {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split the terminal vertically: top bar and main area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Navigation bar height
                Constraint::Min(0),    // Main area
            ])
            .split(area.clone());

        self.nav_bar.render(chunks[0], buf);

        self.section.render(chunks[1], buf);
    }
}

impl UIKeyHandler for State {

    fn handle(&mut self, events: &mut EventHandler, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                events.send(AppEvent::Quit)
            }
            KeyCode::Esc if self.section.index() == 0 => {
                events.send(AppEvent::Quit)
            }
            KeyCode::Right if key_event.modifiers == KeyModifiers::SHIFT => {
                self.section = self.section.next();
                self.nav_bar.navigate(&self.section);
            }
            KeyCode::Left if key_event.modifiers == KeyModifiers::SHIFT => {
                self.section = self.section.prev();
                self.nav_bar.navigate(&self.section);
            }
            // Other handlers you could add here.
            _ => {}
        }
    }
}
