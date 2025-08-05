use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::event::{self, AppEvent, EventSender};

mod chats;
mod home;
mod nav;
mod settings;

pub enum NavSection {
    Home(home::Home),
    Chats(chats::Chats),
    Settings(settings::Settings),
    Help,
}

impl Handler for NavSection {
    fn handle_key(&mut self, event_sender: &mut EventSender, key_event: KeyEvent) -> color_eyre::Result<()> {
        match self {
            NavSection::Home(section) => section.handle_key(event_sender, key_event),
            NavSection::Chats(section) => section.handle_key(event_sender, key_event),
            NavSection::Settings(section) => section.handle_key(event_sender, key_event),
            NavSection::Help => { Ok(()) }
        }
    }
}

impl NavSection {
    pub fn index(&self) -> usize {
        match self {
            NavSection::Home(_) => 0,
            NavSection::Chats(_) => 1,
            NavSection::Settings(_) => 2,
            NavSection::Help => 3,
        }
    }

    pub fn prev(&self, state: &State) -> NavSection {
        match self {
            NavSection::Home(_) => NavSection::Help,
            NavSection::Chats(_) => NavSection::Home(Default::default()),
            NavSection::Settings(_) => NavSection::Chats(Default::default()),
            NavSection::Help => {
                NavSection::Settings(settings::Settings::new(state.local_peer_id.clone()))
            }
        }
    }

    pub fn next(&self, state: &State) -> NavSection {
        match self {
            NavSection::Home(_) => NavSection::Chats(Default::default()),
            NavSection::Chats(_) => {
                NavSection::Settings(settings::Settings::new(state.local_peer_id.clone()))
            }
            NavSection::Settings(_) => NavSection::Help,
            NavSection::Help => NavSection::Home(Default::default()),
        }
    }
}

impl Default for NavSection {
    fn default() -> Self {
        NavSection::Home(Default::default())
    }
}

impl Widget for &NavSection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            NavSection::Home(section) => section.render(area, buf),
            NavSection::Chats(section) => section.render(area, buf),
            NavSection::Settings(section) => section.render(area, buf),
            NavSection::Help => {}
        }
    }
}

pub trait Handler {
    fn handle(&mut self, event_sender: &mut EventSender, event: event::Event) -> color_eyre::Result<()> {
        match event {
            event::Event::App(_) => Ok(()),
            event::Event::Tick => Ok(()),
            event::Event::Crossterm(crossterm_event) => match crossterm_event {
                CrosstermEvent::Key(key_event) => {
                    self.handle_key(event_sender, key_event)
                }
                _ => Ok(()),
            },
        }
    }

    fn handle_key(&mut self, event_sender: &mut EventSender, key_event: KeyEvent) -> color_eyre::Result<()>;
}

pub struct State {
    local_peer_id: libp2p::PeerId,
    nav_bar: nav::NavBar,
    section: NavSection,
}

impl State {
    pub fn new(chat_app: &streuen_chat::ChatApp) -> Self {
        Self {
            local_peer_id: chat_app.current_user().peer_id(),
            nav_bar: Default::default(),
            section: Default::default(),
        }
    }
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

impl Handler for State {
    fn handle(&mut self, event_sender: &mut EventSender, event: crate::event::Event) -> color_eyre::Result<()> {
        self.section.handle(event_sender, event.clone())?;
        match event {
            crate::event::Event::App(_) => Ok(()),
            crate::event::Event::Tick => Ok(()),
            crate::event::Event::Crossterm(crossterm_event) => match crossterm_event {
                CrosstermEvent::Key(key_event) => {
                    self.handle_key(event_sender, key_event)
                }
                _ => Ok(()),
            },
        }

    }

    fn handle_key(&mut self, event_sender: &mut EventSender, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                event_sender.send(AppEvent::Quit)
            }
            KeyCode::Right if key_event.modifiers == KeyModifiers::SHIFT => {
                self.section = self.section.next(&self);
                self.nav_bar.navigate(&self.section);
                Ok(())
            }
            KeyCode::Left if key_event.modifiers == KeyModifiers::SHIFT => {
                self.section = self.section.prev(&self);
                self.nav_bar.navigate(&self.section);
                Ok(())
            }
            _ => { Ok(()) },
        }
    }
}
