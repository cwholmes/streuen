use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split the terminal vertically: top bar and main area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Navigation bar height
                Constraint::Min(0),    // Main area
            ])
            .split(area.clone());

        // Navigation bar (top)
        let nav_titles = [
            ("Home", "<h>"),
            ("Chats", "<c>"),
            ("Settings", "<s>"),
            ("Help", "?"),
        ];
        let nav_spans: Vec<Line> = nav_titles
            .iter()
            .map(|(title, key)| {
                Line::default().spans(vec![
                    Span::raw(*title),
                    Span::raw(" "),
                    Span::styled(*key, Style::default().fg(Color::Blue)),
                ])
            })
            .collect();
        let nav_bar = Block::default()
            .title("Navigation")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow));
        let nav_bar = Tabs::new(nav_spans)
            .block(nav_bar)
            .select(self.nav_index)
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().fg(Color::White));
        nav_bar.render(chunks[0], buf);

        // Split the main area horizontally: left (chat), right (user list)
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15), // User List
                Constraint::Percentage(85), // Chat Box
            ])
            .split(chunks[1]);

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
