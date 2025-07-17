use libp2p::PeerId;
use libp2p::identity::Keypair;

pub struct User {
    name: String,
    keypair: Keypair,
}

impl User {
    pub fn new(name: String) -> Self {
        let keypair = Keypair::generate_ed25519();
        Self { name, keypair }
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
}
