use libp2p::identify;
use libp2p::kad;
use libp2p::relay;
use libp2p::request_response;
use libp2p::swarm::NetworkBehaviour;
use serde::{Deserialize, Serialize};

#[derive(NetworkBehaviour)]
pub struct ChatBehavior {
    pub request_response: request_response::cbor::Behaviour<ChatSendMessage, ChatMessageReceived>,
    pub relay: relay::client::Behaviour,
    pub kad: kad::Behaviour<kad::store::MemoryStore>,
    pub identify: identify::Behaviour,
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
