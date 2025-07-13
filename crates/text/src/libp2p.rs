pub mod behavior;

use libp2p::StreamProtocol;
use libp2p::Transport;
use libp2p::identify;
use libp2p::identity;
use libp2p::kad;
use libp2p::noise;
use libp2p::request_response;
use libp2p::swarm::Swarm;
use libp2p::webrtc_websys;
use libp2p::websocket_websys;
use libp2p::yamux;

use crate::libp2p::behavior::{ChatBehavior, ChatMessageReceived, ChatSendMessage};

#[allow(dead_code)]
pub fn build_swarm() -> Result<Swarm<ChatBehavior>, Box<dyn std::error::Error>> {
    let id_keys = identity::Keypair::generate_ed25519();

    let protocols = [(
        StreamProtocol::new("/decentral-management/chat/send/1.0.0"),
        request_response::ProtocolSupport::Full,
    )];

    let swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
        .with_wasm_bindgen()
        .with_other_transport(|key| {
            webrtc_websys::Transport::new(webrtc_websys::Config::new(&key))
        })?
        .with_other_transport(|key| {
            websocket_websys::Transport::default()
                .upgrade(libp2p::core::upgrade::Version::V1)
                .authenticate(noise::Config::new(&key).unwrap())
                .multiplex(yamux::Config::default())
                .boxed()
        })?
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|key, relay| {
            let local_peer_id = key.public().to_peer_id();
            ChatBehavior {
                request_response: request_response::cbor::Behaviour::<
                    ChatSendMessage,
                    ChatMessageReceived,
                >::new(
                    protocols, request_response::Config::default()
                ),
                relay: relay,
                kad: kad::Behaviour::new(
                    local_peer_id,
                    kad::store::MemoryStore::new(local_peer_id),
                ),
                identify: identify::Behaviour::new(identify::Config::new(
                    "/decentral-management/chat/send/1.0.0".to_string(),
                    key.public(),
                )),
            }
        })?
        .build();

    Ok(swarm)
}
