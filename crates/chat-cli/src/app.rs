use crate::{
    event::EventHandler,
    ui::{self, Handler},
};

use ratatui::DefaultTerminal;

use streuen_chat::app::ToChat;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// Chat App
    pub chat_app: streuen_chat::ChatApp,
    /// UI component state
    pub ui_state: ui::State,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> color_eyre::Result<Self> {
        let chat_app = streuen_chat::ChatApp::new("Me".to_string())?;
        let ui_state = ui::State::new(&chat_app);
        Ok(Self {
            running: true,
            events: EventHandler::new(),
            chat_app,
            ui_state,
        })
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
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
            self.chat_app
                .chat_dispatch(ToChat::AddBoostrapPeer(bootstrap_addr));
        }

        // Listen on all interfaces and whatever port the OS assigns
        self.chat_app
            .chat_dispatch(ToChat::ListenOn("/ip4/0.0.0.0/udp/0/quic-v1".parse()?));
        self.chat_app
            .chat_dispatch(ToChat::ListenOn("/ip4/0.0.0.0/tcp/0".parse()?));

        while self.running {
            terminal.draw(|frame| frame.render_widget(&self.ui_state, frame.area()))?;
            let event = self.events.next().await?;
            if event.should_quit() {
                self.quit();
            } else {
                self.ui_state.handle(&mut self.events.sender(), event)?;
            }
        }
        Ok(())
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
