use std::task::Poll;
use std::time::Duration;

use libp2p::Transport;
use libp2p::gossipsub;
use libp2p::identity;
use libp2p::noise;
use libp2p::webrtc_websys;
use libp2p::websocket_websys;
use libp2p::yamux;

use streuen_chat::libp2p::behavior::ChatBehavior;

const BOOTSTRAP_INTERVAL: Duration = Duration::from_secs(5 * 60);

pub fn build_websys_swarm(keypair: &identity::Keypair) -> Result<(), Box<dyn std::error::Error>> {
    use libp2p::futures::StreamExt;

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair.clone())
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
        .with_behaviour(|keypair, relay| {
            ChatBehavior::new(keypair, Some(relay))
        })?
        .with_swarm_config(|config| config)
        .build();

    let bootstrap_peer: libp2p::PeerId =
        "QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN".parse()?;

    let bootstrap_addr: libp2p::Multiaddr = "/dns4/sv15.bootstrap.libp2p.io/tcp/443/wss/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN".parse()?;

    let _ = swarm
        .behaviour_mut()
        .kad
        .as_mut()
        .map(|k| k.add_address(&bootstrap_peer, bootstrap_addr));

    let topic = gossipsub::IdentTopic::new("test-decentral-management");

    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    let mut bootstrap_timer = futures_timer::Delay::new(BOOTSTRAP_INTERVAL);

    wasm_bindgen_futures::spawn_local(async move {
        loop {
            use libp2p::swarm::SwarmEvent;

            if let Poll::Ready(()) = futures::poll!(&mut bootstrap_timer) {
                bootstrap_timer.reset(BOOTSTRAP_INTERVAL);
                let _ = swarm.behaviour_mut().kad.as_mut().map(|k| k.bootstrap());
            }

            match swarm.select_next_some().await {
                SwarmEvent::Behaviour(event) => {
                    use web_sys::console;

                    console::log_1(&format!("{event:?}").into());
                }
                _ => {}
            }
        }
    });

    Ok(())
}
