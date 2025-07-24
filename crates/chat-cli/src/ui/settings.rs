use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::event::AppEvent;

#[derive(Debug, Clone)]
pub struct Settings {}

impl Default for Settings {
    fn default() -> Self {
        Self {}
    }
}

impl Widget for &Settings {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("streuen-chat-cli")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = "Welcome to Streuen.\n\
                `Ctrl-C` will close the application.\n\
                `Esc` will return to the previous tab unless on the home page, which will close the application.\n\
                Press navigate to the desired tab.";

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(area, buf);
    }
}

impl super::UIKeyHandler for Settings {
    fn handle(&mut self, events: &mut crate::event::EventHandler, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => events.send(AppEvent::Quit),
            _ => {}
        }
    }
}
