use crossterm::event::{KeyCode, KeyEvent};
use libp2p::PeerId;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Borders, Clear, List, ListItem, Widget},
};

use crate::event::{AppEvent, EventSender};

pub struct Settings {
    local_peer_id: PeerId,
    selected: usize,
    show_selected: bool,
}

impl Settings {
    pub fn new(local_peer_id: PeerId) -> Self {
        Self {
            local_peer_id,
            selected: 1,
            show_selected: false
        }
    }

    fn handle_enter_selected(&mut self, num: usize) -> color_eyre::Result<()> {
        match num {
            1 => {
                match cli_clipboard::set_contents(self.local_peer_id.to_base58()) {
                    Ok(_) => {
                        tracing::info!("Copied peer id to clipboard: {}", self.local_peer_id.to_base58());
                    }
                    Err(_) => {
                        tracing::error!("Failed to copy peer id to clipboard: {}", self.local_peer_id.to_base58());
                    }
                }
                Ok(())
            }
            _ => {
                self.selected = num;
                self.show_selected = true;
                Ok(())
            }
        }
    }
}

impl Widget for &Settings {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Settings List
        let settings_options = vec![
            ListItem::new(format!("1: Copy Peer Id: {}", self.local_peer_id.to_base58())),
            ListItem::new("2: Dial Peer"),
        ];
        let user_list =
            List::new(settings_options).block(Block::default().title("Users").borders(Borders::ALL));
        user_list.render(area, buf);

        if self.show_selected {
            let popup_area = area;
            let vertical_layout = Layout::vertical([Constraint::Percentage(20)]).flex(Flex::Center);
            let horizontal_layout = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
            let [popup_area] = vertical_layout.areas(popup_area);
            let [popup_area] = horizontal_layout.areas(popup_area);
            match self.selected {
                2 => {
                    let block = Block::bordered().title("Enter Peer To Be Dialed:");
                    Clear.render(popup_area, buf);
                    block.render(popup_area, buf);
                }
                _ => (),
            }
        }
    }
}

impl super::Handler for Settings {
    fn handle_key(&mut self, event_sender: &mut EventSender, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                if self.show_selected {
                    self.show_selected = false;
                    Ok(())
                } else {
                    event_sender.send(AppEvent::Quit)
                }
            }
            KeyCode::Char(n) => {
                self.show_selected = false;
                if let Some(num) = n.to_digit(10) {
                    self.handle_enter_selected(num as usize)?;
                }
                Ok(())
            }
            KeyCode::Enter => {
                self.handle_enter_selected(self.selected)
            }
            _ => Ok(()),
        }
    }
}
