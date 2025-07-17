use libp2p::PeerId;
use libp2p::identity::Keypair;

#[derive(Clone, Debug)]
pub struct User {
    name: String,
    keypair: Keypair,
}

impl User {
    pub fn new(name: String) -> Self {
        let keypair = Keypair::generate_ed25519();
        Self { name, keypair }
    }

    pub fn keypair(&self) -> Keypair {
        self.keypair.clone()
    }

    pub fn peer_id(&self) -> PeerId {
        self.keypair.public().to_peer_id()
    }
}

pub struct Users {
    users: Vec<User>,
    current_user: User,
}

impl Users {
    pub fn new(current_user: User) -> Self {
        Self {
            users: Vec::new(),
            current_user,
        }
    }

    pub fn current_user(&self) -> User {
        self.current_user.clone()
    }
}
