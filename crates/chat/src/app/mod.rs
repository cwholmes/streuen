pub(crate) mod error;
mod messages;
mod users;

use std::task::{Context, Poll};

use futures_channel::mpsc;

pub struct ChatApp<'a, H: AppHandler> {
    users: users::Users,
    messages: messages::Messages,

    app_handlers: Vec<&'a H>,

    swarm_sender: Option<mpsc::Sender<SwarmEvent>>,
}

impl<'a, H: AppHandler> ChatApp<'a, H> {
    pub fn new(name: String) -> Result<Self, error::ChatAppError> {
        let current_user = users::User::new(name);

        Ok(Self {
            users: users::Users::new(current_user),
            messages: messages::Messages::new(),

            app_handlers: Vec::new(),

            swarm_sender: None,
        })
    }

    pub fn current_user(&self) -> users::User {
        self.users.current_user()
    }

    pub fn start_swarm(&self) -> Result<(), error::ChatAppError> {
        // let swarm_start = crate::libp2p::SwarmStart::new(
        //     self.users.current_user().keypair(),
        //     self.app_sender.clone(),
        //     self.swarm_receiver.clone(),
        // );

        // swarm_start.start_swarm()?;

        Ok(())
    }

    // pub fn send_app_event(&mut self, event: AppEvent) -> Result<(), error::ChatAppError> {
    //     self.app_sender.try_send(event)?;

    //     Ok(())
    // }

    // pub async fn next_app_event(&mut self) -> Option<AppEvent> {
    //     self.app_receiver.next().await
    // }

    // pub fn send_swarm_event(&mut self, event: SwarmEvent) -> Result<(), error::ChatAppError> {
    //     self.swarm_sender.try_send(event)?;

    //     Ok(())
    // }

    // pub async fn next_swarm_event(&mut self) -> Option<SwarmEvent> {
    //     self.swarm_receiver.next().await
    // }

    pub fn register_app_handler(&mut self, handler: H) {
        self.app_handlers.push(Box::new(handler));
    }
}

pub trait AppHandler {
    fn handle(&mut self, event: &AppEvent);

    // fn poll(&mut self, _cx: &mut Context<'_>) -> Poll<SwarmEvent> {
    //     Poll::Pending
    // }
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
