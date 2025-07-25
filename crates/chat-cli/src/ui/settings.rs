use crossterm::event::{KeyCode, KeyEvent};
use libp2p::PeerId;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::event::AppEvent;

pub struct Settings {
    local_peer_id: PeerId,
}

impl Settings {
    pub fn new(local_peer_id: PeerId) -> Self {
        Self { local_peer_id }
    }
}

impl Widget for &Settings {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("streuen-chat-cli")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "Welcome to Streuen.\n\
                `Ctrl-C` will close the application.\n\
                `Esc` will return to the previous tab unless on the home page, which will close the application.\n\
                local_peer_id = {}",
            self.local_peer_id,
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(area, buf);
    }
}

impl super::Handler for Settings {
    fn handle_key(&mut self, events: &mut crate::event::EventHandler, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => events.send(AppEvent::Quit),
            _ => {}
        }
    }
}
