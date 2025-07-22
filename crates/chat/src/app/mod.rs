pub(crate) mod error;
mod messages;
mod users;

use std::rc::Rc;

use futures::channel::mpsc;
use libp2p::{Multiaddr, PeerId, identity::Keypair};

pub use crate::libp2p::behaviour::{InnerChatBehavior, ToChat};

pub struct ChatApp {
    users: users::Users,
    messages: messages::Messages,

    app_callbacks: Vec<AppCallback>,

    chat_behavior: InnerChatBehavior,
}

impl ChatApp {
    pub fn new(name: String) -> Result<Self, error::ChatAppError> {
        let keypair = Keypair::generate_ed25519();
        let current_user = users::User::new(name, keypair.public().to_peer_id().clone());
        let users = users::Users::new(current_user, keypair.clone());

        let chat_behavior = crate::libp2p::run_swarm(keypair.clone())?;

        Ok(Self {
            users,
            messages: messages::Messages::new(),

            app_callbacks: Vec::new(),

            chat_behavior,
        })
    }

    pub(crate) fn keypair(&self) -> Keypair {
        self.users.keypair()
    }

    pub fn current_user(&self) -> users::User {
        self.users.current_user().as_ref().clone()
    }

    pub fn chat_dispatch(&mut self, event: ToChat) {
        self.chat_behavior.send(event);
    }

    pub fn register_app_handler(&mut self, cb: AppCallback) {
        self.app_callbacks.push(cb);
    }
}

pub struct AppCallback {
    cb: Rc<dyn Fn(ToApp) -> ()>,
}

impl AppCallback {
    pub fn emit(&self, event: ToApp) {
        (*self.cb)(event)
    }
}

impl<F: Fn(ToApp) -> () + 'static> From<F> for AppCallback {
    fn from(func: F) -> Self {
        Self { cb: Rc::new(func) }
    }
}

impl Clone for AppCallback {
    fn clone(&self) -> Self {
        Self {
            cb: self.cb.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ToApp {
    X,
}
