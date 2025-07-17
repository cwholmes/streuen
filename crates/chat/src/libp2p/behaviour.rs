use libp2p::gossipsub;
use libp2p::identity;
use libp2p::kad;
use libp2p::relay;
use libp2p::request_response;
use libp2p::swarm::NetworkBehaviour;
use libp2p::swarm::behaviour::toggle::Toggle;
use serde::{Deserialize, Serialize};

#[derive(NetworkBehaviour)]
pub struct ChatBehaviour {
    // this req/rep model isn't going to work and will be replaced by either pubsub or a custom protocol
    pub request_response: request_response::cbor::Behaviour<ChatSendMessage, ChatMessageReceived>,
    relay_client: Toggle<relay::client::Behaviour>,
    pub kad: Toggle<kad::Behaviour<kad::store::MemoryStore>>,
    pub gossipsub: gossipsub::Behaviour,
}

impl ChatBehaviour {
    pub fn new(
        keypair: &identity::Keypair,
        relay_client: Option<relay::client::Behaviour>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let local_peer_id = keypair.public().to_peer_id();

        let protocols = [(
            super::CHAT_PROTOCOL,
            request_response::ProtocolSupport::Full,
        )];

        let request_response = request_response::cbor::Behaviour::<
            ChatSendMessage,
            ChatMessageReceived,
        >::new(protocols, request_response::Config::default());

        let kad = kad::Behaviour::new(local_peer_id, kad::store::MemoryStore::new(local_peer_id));

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(keypair.clone()),
            gossipsub::Config::default(),
        )?;

        Ok(Self {
            request_response,
            relay_client: relay_client.into(),
            kad: Some(kad).into(),
            gossipsub,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatSendMessage {
    pub message_id: u8,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatMessageReceived {
    message_id: u8,
}
