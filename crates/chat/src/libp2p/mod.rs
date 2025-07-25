pub(crate) mod behaviour;

use libp2p::mdns;
use libp2p::{
    StreamProtocol, Swarm, SwarmBuilder, multiaddr, noise, request_response, swarm::SwarmEvent,
    yamux,
};

use crate::app;
use crate::libp2p::behaviour::ChatBehaviourEvent;

const CHAT_PROTOCOL: StreamProtocol = StreamProtocol::new("/streuen/chat/0.1.0");

pub(crate) fn run_swarm(
    keypair: libp2p::identity::Keypair,
) -> Result<behaviour::InnerChatBehavior, app::error::ChatAppError> {
    let swarm = build_swarm(keypair)?;

    let inner_behavior = swarm.behaviour().inner.clone();

    spawn_swarm_loop(run_swarm_loop(swarm));

    Ok(inner_behavior)
}

async fn run_swarm_loop(mut swarm: Swarm<behaviour::ChatBehaviour>) {
    use libp2p::futures::StreamExt;

    loop {
        if let Some(event) = swarm.next().await {
            match event {
                SwarmEvent::Behaviour(ref behavior_event) => match behavior_event {
                    ChatBehaviourEvent::Inner(behaviour::ChatToSwarm::ListenOn(addr)) => {
                        if let Err(err) = swarm.listen_on(addr.clone()) {
                            tracing::error!("Error listening to address [{addr}]: {err:?}")
                        } else {
                            tracing::info!("Listening to address: {addr}")
                        }
                    }
                    ChatBehaviourEvent::Inner(behaviour::ChatToSwarm::AddBoostrapPeer(addr)) => {
                        if let Some(multiaddr::Protocol::P2p(peer_id)) = addr.iter().last() {
                            swarm.behaviour_mut().kad.as_mut().map(|k| {
                                let _ = k.add_address(&peer_id, addr.clone());
                                let _ = k.bootstrap();
                            });
                        } else {
                            tracing::error!("Invalid bootstrap address: {addr}")
                        }
                    }
                    ChatBehaviourEvent::RequestResponse(request_response::Event::Message {
                        peer,
                        connection_id: _,
                        message,
                    }) => {
                        tracing::debug!("Received message: {peer} {message:?}")
                    }
                    ChatBehaviourEvent::Mdns(mdns::Event::Discovered(peers)) => {
                        for (peer_id, _addr) in peers {
                            tracing::info!("Peer discovered from mDNS: {peer_id}");
                            if swarm.is_connected(peer_id) {
                                continue;
                            }
                            let _ = swarm.dial(peer_id.clone());
                        }
                    }
                    _ => tracing::debug!("{event:?}"),
                },
                event => tracing::info!("Swarm Event: {event:?}"),
            }
        }

        swarm
            .external_addresses()
            .for_each(|a| tracing::info!("External address: {a}"));
    }
}

#[cfg(target_arch = "wasm32")]
fn build_swarm(
    keypair: libp2p::identity::Keypair,
) -> Result<Swarm<behaviour::ChatBehaviour>, app::error::ChatAppError> {
    use libp2p::Transport;
    use libp2p::webrtc_websys;
    use libp2p::websocket_websys;

    let builder = SwarmBuilder::with_existing_identity(keypair)
        .with_wasm_bindgen()
        .with_other_transport(|key| webrtc_websys::Transport::new(webrtc_websys::Config::new(&key)))
        .unwrap() // this is Infallible so this is safe
        .with_other_transport(|key| {
            websocket_websys::Transport::default()
                .upgrade(libp2p::core::upgrade::Version::V1)
                .authenticate(noise::Config::new(&key).unwrap())
                .multiplex(yamux::Config::default())
                .boxed()
        })
        .unwrap() // this is Infallible so this is safe
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|keypair, relay| behaviour::ChatBehaviour::new(keypair, Some(relay)))
        .unwrap(); // this is Infallible so this is safe

    Ok(builder.build())
}

#[cfg(target_arch = "wasm32")]
fn spawn_swarm_loop<F: Future<Output = ()> + Send + 'static>(future: F) {
    wasm_bindgen_futures::spawn_local(future);
}

#[cfg(not(target_arch = "wasm32"))]
fn build_swarm(
    keypair: libp2p::identity::Keypair,
) -> Result<Swarm<behaviour::ChatBehaviour>, app::error::ChatAppError> {
    let builder = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::new(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_dns()?
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|keypair, relay| behaviour::ChatBehaviour::new(keypair, Some(relay)))
        .unwrap(); // this is Infallible so this is safe

    Ok(builder.build())
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_swarm_loop<F: Future<Output = ()> + Send + 'static>(future: F) {
    tokio::task::spawn(future);
}
