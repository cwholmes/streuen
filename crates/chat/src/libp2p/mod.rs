mod behaviour;

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

use futures_channel::mpsc;
use libp2p::{
    StreamProtocol, Swarm, SwarmBuilder, multiaddr, noise,
    request_response, swarm::SwarmEvent, yamux,
};

use crate::app;
use crate::libp2p::behaviour::ChatBehaviourEvent;

const CHAT_PROTOCOL: StreamProtocol = StreamProtocol::new("/streuen/chat/0.1.0");

pub(crate) struct SwarmRunner<'a> {
    chat_app: &'a app::ChatApp,
}

impl<'a> SwarmRunner<'a> {
    pub fn new(chat_app: &'a app::ChatApp) -> Self {
        Self { chat_app }
    }

    pub(crate) fn run_swarm(&mut self, swarm_queue: Rc<RefCell<VecDeque<app::SwarmEvent>>>) -> Result<(), app::error::ChatAppError> {
        let swarm = self.build_swarm()?;

        let swarm_loop = SwarmRunner::run_swarm_loop(swarm_queue, swarm);

        SwarmRunner::spawn_swarm_loop(swarm_loop);

        Ok(())
    }

    async fn run_swarm_loop(
        swarm_queue: Rc<RefCell<VecDeque<app::SwarmEvent>>>,
        mut swarm: Swarm<behaviour::ChatBehaviour>,
    ) {
        use libp2p::futures::StreamExt;

        loop {
            if let Some(event) = swarm_queue.borrow_mut().pop_front() {
                match event {
                    app::SwarmEvent::AddBoostrapPeer(addr) => {
                        if let Some(multiaddr::Protocol::P2p(peer_id)) = addr.iter().last() {
                            swarm.behaviour_mut().kad.as_mut().map(|k| {
                                let _ = k.add_address(&peer_id, addr);
                                let _ = k.bootstrap();
                            });
                        }
                    }
                    app::SwarmEvent::Connect(peer_id) => match swarm.dial(peer_id) {
                        Ok(_) => tracing::debug!("Connection to peer successful: {peer_id}"),
                        Err(err) => tracing::debug!("Connection to peer failed: {err:?}"),
                    },
                    app::SwarmEvent::SendMessage(peer_id, message) => {
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
impl<'a> SwarmRunner<'a> {

    fn build_swarm(&self) -> Result<Swarm<behaviour::ChatBehaviour>, app::error::ChatAppError> {
        use libp2p::Transport;
        use libp2p::webrtc_websys;
        use libp2p::websocket_websys;

        let builder = SwarmBuilder::with_existing_identity(self.chat_app.keypair())
            .with_wasm_bindgen()
            .with_other_transport(|key| {
                webrtc_websys::Transport::new(webrtc_websys::Config::new(&key))
            })
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

    fn spawn_swarm_loop<F: Future<Output = ()> + 'static>(future: F) {
        wasm_bindgen_futures::spawn_local(future);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> SwarmRunner<'a> {

    fn build_swarm(&self) -> Result<Swarm<behaviour::ChatBehaviour>, app::error::ChatAppError> {
        let builder = SwarmBuilder::with_existing_identity(self.chat_app.keypair())
            .with_tokio()
            .with_tcp(
                libp2p::tcp::Config::new(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_relay_client(noise::Config::new, yamux::Config::default)?
            .with_behaviour(|keypair, relay| behaviour::ChatBehaviour::new(keypair, Some(relay)))
            .unwrap(); // this is Infallible so this is safe

        Ok(builder.build())
    }

    fn spawn_swarm_loop<F: Future<Output = ()> + 'static>(future: F) {
        ()
    }
}
