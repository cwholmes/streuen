use libp2p::{
    Multiaddr, PeerId,
    core::{Endpoint, transport::PortUse},
    gossipsub,
    identity::Keypair,
    kad, relay, request_response,
    swarm::{
        ConnectionDenied, ConnectionId, FromSwarm, NetworkBehaviour, THandler, THandlerInEvent,
        THandlerOutEvent, ToSwarm, behaviour::toggle::Toggle, dummy,
    },
};
use serde::{Deserialize, Serialize};

use std::{
    collections::VecDeque,
    task::{Context, Poll},
    sync::{Arc, Mutex},
};

use crate::app;

#[derive(Clone, Debug)]
pub enum ToChat {
    AddBoostrapPeer(Multiaddr),
    Connect(PeerId),
    SendMessage(PeerId, String),
}

#[derive(Clone, Debug)]
pub enum ChatToSwarm {
    AddBoostrapPeer(Multiaddr),
}

#[derive(Clone)]
pub struct InnerChatBehavior {
    queue: Arc<Mutex<VecDeque<ToChat>>>,
}

impl InnerChatBehavior {
    pub fn send(&mut self, event: ToChat) {
        self.queue.lock().unwrap().push_back(event);
    }
}

impl NetworkBehaviour for InnerChatBehavior {
    type ConnectionHandler = dummy::ConnectionHandler;
    type ToSwarm = ChatToSwarm;

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        _peer: PeerId,
        _local_addr: &Multiaddr,
        _remote_addr: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(dummy::ConnectionHandler)
    }

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        _peer: PeerId,
        _addr: &Multiaddr,
        _role_override: Endpoint,
        _port_use: PortUse,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(dummy::ConnectionHandler)
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {}

    fn on_connection_handler_event(
        &mut self,
        _peer_id: PeerId,
        _connection_id: ConnectionId,
        _event: THandlerOutEvent<Self>,
    ) {
    }

    fn poll(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        let mut queue = self.queue.lock().unwrap();
        if let Some(event) = queue.pop_front() {
            match event {
                ToChat::AddBoostrapPeer(addr) => {
                    return Poll::Ready(ToSwarm::GenerateEvent(ChatToSwarm::AddBoostrapPeer(addr)));
                }
                ToChat::Connect(peer_id) => {
                    return Poll::Ready(ToSwarm::Dial { opts: peer_id.into() });
                }
                ToChat::SendMessage(peer_id, message) => {
                    // let request = behaviour::ChatSendMessage {
                    //     message_id: 1,
                    //     message,
                    // };
                    // swarm
                    //     .behaviour_mut()
                    //     .request_response
                    //     .send_request(&peer_id, request);
                    tracing::debug!("Sending message to peer [{peer_id}]: {message}")
                }
            }
        }
        Poll::Pending
    }
}

#[derive(NetworkBehaviour)]
pub struct ChatBehaviour {
    // this req/rep model isn't going to work and will be replaced by either pubsub or a custom protocol
    pub request_response: request_response::cbor::Behaviour<ChatSendMessage, ChatMessageReceived>,
    relay_client: Toggle<relay::client::Behaviour>,
    pub kad: Toggle<kad::Behaviour<kad::store::MemoryStore>>,
    pub gossipsub: gossipsub::Behaviour,
    pub inner: InnerChatBehavior,
}

impl ChatBehaviour {
    pub fn new(
        keypair: &Keypair,
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
            inner: InnerChatBehavior {
                queue: Arc::new(Mutex::new(VecDeque::new())),
            },
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
