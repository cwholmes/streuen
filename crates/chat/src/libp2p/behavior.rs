use libp2p::StreamProtocol;
use libp2p::gossipsub;
use libp2p::identify;
use libp2p::identity;
use libp2p::kad;
use libp2p::relay;
use libp2p::request_response;
use libp2p::swarm::NetworkBehaviour;
use libp2p::swarm::behaviour::toggle::Toggle;
use serde::{Deserialize, Serialize};

#[derive(NetworkBehaviour)]
pub struct ChatBehavior {
    pub request_response: request_response::cbor::Behaviour<ChatSendMessage, ChatMessageReceived>,
    relay_client: Toggle<relay::client::Behaviour>,
    pub kad: Toggle<kad::Behaviour<kad::store::MemoryStore>>,
    identify: identify::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
}

impl ChatBehavior {
    pub fn new(keypair: &identity::Keypair, relay_client: Option<relay::client::Behaviour>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let local_peer_id = keypair.public().to_peer_id();

        let protocols = [(
            StreamProtocol::new("/decentral-management/chat/send/1.0.0"),
            request_response::ProtocolSupport::Full,
        )];

        let request_response = request_response::cbor::Behaviour::<
            ChatSendMessage,
            ChatMessageReceived,
        >::new(protocols, request_response::Config::default());

        let kad = kad::Behaviour::new(local_peer_id, kad::store::MemoryStore::new(local_peer_id));

        let identify = identify::Behaviour::new(identify::Config::new(
            "/decentral-management/chat/send/1.0.0".to_string(),
            keypair.public(),
        ));

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(keypair.clone()),
            gossipsub::Config::default(),
        )?;

        Ok(Self {
            request_response: request_response,
            relay_client: relay_client.into(),
            kad: Some(kad).into(),
            identify: identify,
            gossipsub: gossipsub,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatSendMessage {
    message_id: u8,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatMessageReceived {
    message_id: u8,
}
