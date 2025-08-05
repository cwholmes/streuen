use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

use crate::event::{AppEvent, EventSender};

#[derive(Default)]
pub struct Chats {}

impl Widget for &Chats {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split the main area horizontally: left (chat), right (user list)
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15), // User List
                Constraint::Percentage(85), // Chat Box
            ])
            .split(area);

        // User list (left)
        let users = vec![
            ListItem::new("Alice"),
            ListItem::new("Bob"),
            ListItem::new("Carol"),
        ];
        let user_list =
            List::new(users).block(Block::default().title("Users").borders(Borders::ALL));
        user_list.render(main_chunks[0], buf);

        // Chat box (right)
        let chat_box = Paragraph::new("Chat messages go here")
            .block(Block::default().title("Chat").borders(Borders::ALL));
        chat_box.render(main_chunks[1], buf);
    }
}

impl super::Handler for Chats {
    fn handle_key(&mut self, event_sender: &mut EventSender, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc => event_sender.send(AppEvent::Quit),
            _ => Ok(()),
        }
    }
}
