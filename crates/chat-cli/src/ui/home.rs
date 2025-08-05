use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::event::{AppEvent, EventSender};

#[derive(Default)]
pub struct Home {}

impl Widget for &Home {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("streuen-chat-cli")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = "Welcome to Streuen.\n\
                `Ctrl-C` will close the application.\n\
                `Esc` will return to the previous tab unless on the home page, which will close the application.\n\
                Please navigate to the desired tab.";

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(area, buf);
    }
}

impl super::Handler for Home {
    fn handle_key(&mut self, event_sender: &mut EventSender, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc => event_sender.send(AppEvent::Quit),
            _ => Ok(()),
        }
    }
}
