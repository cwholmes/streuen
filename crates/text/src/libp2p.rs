use libp2p::StreamProtocol;
use libp2p::identity;
use libp2p::request_response;
use libp2p::swarm::NetworkBehaviour;
use libp2p::swarm::Swarm;
use libp2p::webrtc_websys;
use serde::{Deserialize, Serialize};

#[derive(NetworkBehaviour)]
pub(crate) struct ChatBehavior {
    request_response: request_response::cbor::Behaviour<ChatSendMessage, ChatMessageReceived>,
}

#[allow(dead_code)]
pub(crate) fn build_swarm() -> Result<Swarm<ChatBehavior>, Box<dyn std::error::Error>> {
    let id_keys = identity::Keypair::generate_ed25519();

    let protocols = [(
        StreamProtocol::new("/decentral-management/chat/send/1.0.0"),
        request_response::ProtocolSupport::Full,
    )];

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
        .with_wasm_bindgen()
        .with_other_transport(|key| {
            webrtc_websys::Transport::new(webrtc_websys::Config::new(&key))
        })?
        .with_behaviour(|_| ChatBehavior {
            request_response: request_response::cbor::Behaviour::<
                ChatSendMessage,
                ChatMessageReceived,
            >::new(protocols, request_response::Config::default()),
        })?
        .build();

    Ok(swarm)
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct ChatSendMessage {
    message_id: u8,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct ChatMessageReceived {
    message_id: u8,
}
