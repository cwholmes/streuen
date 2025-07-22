use std::rc::Rc;

use libp2p::bytes::BufMut;
use libp2p::PeerId;
use libp2p::identity::Keypair;

#[derive(Clone, Debug)]
pub struct CurrentUser {
    user: Rc<User>,
    keypair: Keypair,
}

#[derive(Clone, Debug)]
pub struct User {
    name: String,
    peer_id: PeerId,
}

impl User {
    pub fn new(name: String, peer_id: PeerId) -> Self {
        Self { name, peer_id }
    }

    pub fn peer_id(&self) -> PeerId {
        self.peer_id.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Users {
    users: Vec<Rc<User>>,
    current_user: CurrentUser,
}

impl Users {
    pub fn new(current_user: User, keypair: Keypair) -> Self {
        let current_user = Rc::new(current_user);
        let current_user = CurrentUser {
            user: current_user,
            keypair,
        };

        let mut users = Vec::new();
        users.push(current_user.user.clone());

        Self {
            users,
            current_user,
        }
    }

    pub(crate) fn keypair(&self) -> Keypair {
        self.current_user.keypair.clone()
    }

    pub(crate) fn current_user(&self) -> Rc<User> {
        self.current_user.user.clone()
    }
}
