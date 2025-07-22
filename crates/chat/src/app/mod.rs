pub(crate) mod error;
mod messages;
mod users;

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

use futures_channel::mpsc;
use libp2p::{identity::Keypair, Multiaddr, PeerId};

pub struct ChatApp {
    users: users::Users,
    messages: messages::Messages,

    app_callbacks: Vec<AppCallback>,

    swarm_queue: Rc<RefCell<VecDeque<SwarmEvent>>>,
}

impl ChatApp {
    pub fn new(name: String) -> Self {
        let keypair = Keypair::generate_ed25519();
        let current_user = users::User::new(name, keypair.public().to_peer_id().clone());
        let users = users::Users::new(current_user, keypair);

        Self {
            users,
            messages: messages::Messages::new(),

            app_callbacks: Vec::new(),

            swarm_queue: Default::default(),
        }
    }

    pub(crate) fn keypair(&self) -> Keypair {
        self.users.keypair()
    }

    pub fn current_user(&self) -> users::User {
        self.users.current_user().as_ref().clone()
    }

    pub fn connect_to_peer(&mut self, peer_id: PeerId) {
        self.swarm_queue.borrow_mut().push_back(SwarmEvent::Connect(peer_id));
    }

    pub fn start(&mut self) -> Result<(), error::ChatAppError> {
        let mut swarm_runner = crate::libp2p::SwarmRunner::new(self);

        swarm_runner.run_swarm(self.swarm_queue.clone())?;

        Ok(())
    }

    pub fn swarm_dispatch(&mut self, event: SwarmEvent) {
        self.swarm_queue.borrow_mut().push_back(event);
    }

    pub fn dispatcher(&self) -> SwarmDispatcher {
        SwarmDispatcher { swarm_queue: self.swarm_queue.clone() }
    }

    pub fn register_app_handler(&mut self, cb: AppCallback) {
        self.app_callbacks.push(cb);
    }
}

pub struct AppCallback {
    cb: Rc<dyn Fn(AppEvent) -> ()>,
}

impl AppCallback {
    pub fn emit(&self, event: AppEvent) {
        (*self.cb)(event)
    }
}

impl<F: Fn(AppEvent) -> () + 'static> From<F> for AppCallback {
    fn from(func: F) -> Self {
        Self { cb: Rc::new(func) }
    }
}

impl Clone for AppCallback {
    fn clone(&self) -> Self {
        Self { cb: self.cb.clone() }
    }
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    X,
}

pub struct SwarmDispatcher {
    swarm_queue: Rc<RefCell<VecDeque<SwarmEvent>>>,
}

impl SwarmDispatcher {
    pub fn dispatch(&mut self, event: SwarmEvent) {
        self.swarm_queue.borrow_mut().push_back(event);
    }
}

#[derive(Clone, Debug)]
pub enum SwarmEvent {
    AddBoostrapPeer(Multiaddr),
    Connect(PeerId),
    SendMessage(PeerId, String),
}
