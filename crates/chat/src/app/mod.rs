mod error;
mod messages;
mod users;

use futures::StreamExt;
use futures_channel::mpsc;

pub struct ChatApp {
    users: users::Users,
    messages: messages::Messages,

    app_sender: mpsc::Sender<AppEvent>,
    app_receiver: mpsc::Receiver<AppEvent>,

    swarm_sender: mpsc::Sender<SwarmEvent>,
    swarm_receiver: mpsc::Receiver<SwarmEvent>,
}

impl ChatApp {
    pub fn new(name: String) -> Result<Self, error::ChatAppError> {
        let current_user = users::User::new(name);
        let (app_sender, app_receiver) = mpsc::channel(16);
        let (swarm_sender, swarm_receiver) = mpsc::channel(16);

        Ok(Self {
            users: users::Users::new(current_user),
            messages: messages::Messages::new(),

            app_sender,
            app_receiver,

            swarm_sender,
            swarm_receiver,
        })
    }

    pub fn start_swarm(&self) -> Result<(), error::ChatAppError> {
        let swarm_start = crate::libp2p::SwarmStart::new(
            self.users.current_user().keypair(),
            self.app_sender.clone(),
            self.swarm_receiver.clone(),
        );

        swarm_start.start_swarm()?;
    }

    pub fn send_app_event(&mut self, event: AppEvent) -> Result<(), error::ChatAppError> {
        self.app_sender.try_send(event)?;

        Ok(())
    }

    pub async fn next_app_event(&mut self) -> Option<AppEvent> {
        self.app_receiver.next().await
    }

    pub fn send_swarm_event(&mut self, event: SwarmEvent) -> Result<(), error::ChatAppError> {
        self.swarm_sender.try_send(event)?;

        Ok(())
    }

    pub async fn next_swarm_event(&mut self) -> Option<SwarmEvent> {
        self.swarm_receiver.next().await
    }
}

struct AppSender {
    app_sender: mpsc::Sender<AppEvent>,
}

impl AppSender {
    pub fn send(&mut self, event: AppEvent) -> Result<(), error::ChatAppError> {
        self.app_sender.try_send(event)?;

        Ok(())
    }
}

struct SwarmSender {
    swarm_sender: mpsc::Sender<SwarmEvent>,
}

impl SwarmSender {
    pub fn send(&mut self, event: SwarmEvent) -> Result<(), error::ChatAppError> {
        self.swarm_sender.try_send(event)?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AppEvent {
    X,
}

#[derive(Clone, Debug)]
pub(crate) enum SwarmEvent {
    Y,
}
