use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs, Widget},
};

#[derive(Debug)]
pub struct NavBar {
    index: usize,
}

impl NavBar {
    pub fn navigate(&mut self, section: &super::NavSection) {
        self.index = section.index();
    }
}

impl Default for NavBar {
    fn default() -> Self {
        Self { index: 0 }
    }
}

impl Widget for &NavBar {

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
            ("Help", "<?>"),
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
            .select(self.index)
            .highlight_style(
                Style::default()
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().fg(Color::White));
        nav_bar.render(chunks[0], buf);
    }
}
