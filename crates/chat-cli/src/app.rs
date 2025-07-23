use crate::event::{AppEvent, Event, EventHandler};

use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

use streuen_chat::app::ToChat;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Nav Bar Index.
    pub nav_index: usize,
    /// Event handler.
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            nav_index: 0,
            events: EventHandler::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        let mut chat_app = streuen_chat::ChatApp::new("Me".to_string())?;

        let bootstrap_peers = [
            "QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
            "QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
            "QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
            "QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
        ];

        for bootstrap_peer in bootstrap_peers {
            let bootstrap_addr = format!("/dnsaddr/bootstrap.libp2p.io/p2p/{bootstrap_peer}")
                .parse()
                .unwrap();
            chat_app.chat_dispatch(ToChat::AddBoostrapPeer(bootstrap_addr));
        }

        chat_app.chat_dispatch(ToChat::ListenOn("/ip4/0.0.0.0/tcp/4001".parse()?));

        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        // self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        // self.counter = self.counter.saturating_sub(1);
    }
}
