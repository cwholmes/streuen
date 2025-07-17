use std::task::Poll;
use std::time::Duration;

use futures_channel::mpsc;
use libp2p::{
    Multiaddr, PeerId, StreamProtocol, Swarm, SwarmBuilder, identity::Keypair, multiaddr, noise,
    request_response, swarm::SwarmEvent, yamux,
};

use crate::app;
use crate::libp2p::behaviour::ChatBehaviourEvent;

mod behaviour;

const CHAT_PROTOCOL: StreamProtocol = StreamProtocol::new("/streuen/chat/0.1.0");

#[derive(Debug, Clone)]
pub enum ChatMsgSend {
    AddBoostrapPeer(Multiaddr),
    Connect(PeerId),
    SendMessage(PeerId, String),
}

#[derive(Debug, Clone)]
pub enum ChatMsgReceive {
    None,
}

pub struct SwarmStart {
    keypair: Keypair,
    app_sender: mpsc::Sender<app::AppEvent>,
    swarm_receiver: mpsc::Receiver<app::SwarmEvent>,
}

impl SwarmStart {
    pub fn new(
        keypair: Keypair,
        app_sender: mpsc::Sender<app::AppEvent>,
        swarm_receiver: mpsc::Receiver<app::SwarmEvent>,
    ) -> Self {
        Self {
            keypair,
            app_sender,
            swarm_receiver,
        }
    }

    async fn run_swarm(&self, mut swarm: Swarm<behaviour::ChatBehaviour>) {
        use libp2p::futures::StreamExt;

        let bootstrap_interval = Duration::from_secs(5 * 60);
        let mut bootstrap_timer = futures_timer::Delay::new(bootstrap_interval);

        loop {
            if let Poll::Ready(()) = futures::poll!(&mut bootstrap_timer) {
                bootstrap_timer.reset(bootstrap_interval);
                let _ = swarm.behaviour_mut().kad.as_mut().map(|k| k.bootstrap());
            }

            if let Some(event) = swarm_receiver.next().await {
                match event {
                    ChatMsgSend::AddBoostrapPeer(addr) => {
                        if let Some(multiaddr::Protocol::P2p(peer_id)) = addr.iter().last() {
                            swarm.behaviour_mut().kad.as_mut().map(|k| {
                                let _ = k.add_address(&peer_id, addr);
                                let _ = k.bootstrap();
                            });
                        }
                    }
                    ChatMsgSend::Connect(peer_id) => match swarm.dial(peer_id) {
                        Ok(_) => tracing::debug!("Connection to peer successful: {peer_id}"),
                        Err(err) => tracing::debug!("Connection to peer failed: {err:?}"),
                    },
                    ChatMsgSend::SendMessage(peer_id, message) => {
                        let request = behaviour::ChatSendMessage {
                            message_id: 1,
                            message,
                        };
                        swarm
                            .behaviour_mut()
                            .request_response
                            .send_request(&peer_id, request);
                    }
                }
            }

            if let Some(event) = swarm.next().await {
                match event {
                    SwarmEvent::Behaviour(ref behavior_event) => match behavior_event {
                        ChatBehaviourEvent::RequestResponse(request_response::Event::Message {
                            peer,
                            connection_id: _,
                            message,
                        }) => {
                            tracing::debug!("Received message: {peer} {message:?}")
                        }
                        _ => tracing::debug!("{event:?}"),
                    },
                    _ => {}
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl SwarmStart {
    pub fn start_swarm(
        &self,
    ) -> Result<
        (mpsc::Sender<ChatMsgSend>, mpsc::Receiver<ChatMsgReceive>),
        Box<dyn std::error::Error + Sync + Send>,
    > {
        use libp2p::Transport;
        use libp2p::webrtc_websys;
        use libp2p::websocket_websys;

        let swarm = SwarmBuilder::with_existing_identity(self.keypair.clone())
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
            .with_behaviour(|keypair, relay| behaviour::ChatBehaviour::new(keypair, Some(relay)))?
            .build();

        wasm_bindgen_futures::spawn_local(self.run_swarm(swarm));

        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl SwarmStart {
    pub fn start_swarm(&self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let swarm = SwarmBuilder::with_existing_identity(self.keypair.clone())
            .with_tokio()
            .with_tcp(
                libp2p::tcp::Config::new(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_relay_client(noise::Config::new, yamux::Config::default)?
            .with_behaviour(|keypair, relay| behaviour::ChatBehaviour::new(keypair, Some(relay)))?
            .build();

        tokio::spawn(self.run_swarm(swarm));

        Ok(())
    }
}
