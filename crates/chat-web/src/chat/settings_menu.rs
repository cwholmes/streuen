use std::str::FromStr;

use libp2p::PeerId;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum SettingsMenuMsg {
    CopyPeerIdToClipboard,
    CloseSettings,
    InputConnectPeer(String),
    ConnectToPeer,
}

#[derive(Properties, PartialEq)]
pub struct SettingsMenuProps {
    pub peer_id: PeerId,
    pub on_close: Callback<()>,
    pub bootstrap: Callback<libp2p::Multiaddr>,
    pub connect: Callback<PeerId>,
}

pub struct SettingsMenu {
    connect_string: String,
}

impl Component for SettingsMenu {
    type Message = SettingsMenuMsg;
    type Properties = SettingsMenuProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            connect_string: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SettingsMenuMsg::CopyPeerIdToClipboard => {
                let clipboard = web_sys::window()
                    .expect("global window does not exist")
                    .navigator()
                    .clipboard();
                let promise = clipboard.write_text(&ctx.props().peer_id.to_base58());
                wasm_bindgen_futures::spawn_local(async move {
                    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
                    match result {
                        Ok(_) => tracing::debug!("Copied peer id to clipboard!"),
                        Err(_) => tracing::debug!("Failed to copy peer id to clipboard!"),
                    }
                });
                false
            }
            SettingsMenuMsg::CloseSettings => {
                ctx.props().on_close.emit(());
                false
            }
            SettingsMenuMsg::InputConnectPeer(peer_string) => {
                self.connect_string = peer_string;
                true
            }
            SettingsMenuMsg::ConnectToPeer => {
                tracing::debug!("Connect to peer: {}", self.connect_string);
                if let Ok(addr) = libp2p::Multiaddr::from_str(&self.connect_string) {
                    ctx.props().bootstrap.emit(addr);
                }
                if let Ok(peer_id) = libp2p::PeerId::from_str(&self.connect_string) {
                    ctx.props().connect.emit(peer_id);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_settings = ctx.link().callback(|_| SettingsMenuMsg::CloseSettings);
        let copy_peer_id_to_clipboard = ctx
            .link()
            .callback(|_| SettingsMenuMsg::CopyPeerIdToClipboard);
        let on_connect_input = ctx.link().callback(|e: InputEvent| {
            let input: Option<HtmlInputElement> = e.target_dyn_into();
            SettingsMenuMsg::InputConnectPeer(input.map(|i| i.value()).unwrap_or_default())
        });

        let connect_to_peer_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            SettingsMenuMsg::ConnectToPeer
        });
        let connect_to_peer_click = ctx.link().callback(|_| SettingsMenuMsg::ConnectToPeer);
        html! {
            <>
                <div class="streuen-chat-settings-window">
                    <div class="streuen-chat-settings-header" style="position: relative;">
                        <p style="margin: 0;">{ "Settings" }</p>
                        <button onclick={close_settings} style="position: absolute; top: 0.5rem; right: 0.5rem; background: #36393f; color: #fff; border: none; border-radius: 50%; width: 1.5rem; height: 1.5rem; font-size: 1rem; font-weight: bold; cursor: pointer; display: flex; align-items: center; justify-content: center; transition: background 0.15s;">
                            { "✕" }
                        </button>
                    </div>
                    <div class="streuen-settings-row" style="position: relative">
                        <p>{ "PeerId: "}</p>
                        <p>{ ctx.props().peer_id.to_base58() }</p>
                        <button class="streuen-settings-bubble" onclick={copy_peer_id_to_clipboard}>{ "Copy" }</button>
                    </div>
                    <div class="streuen-settings-row" style="position: relative">
                        <form class="streuen-chat-add-user-form" onsubmit={connect_to_peer_submit}>
                            <p>{ "Connect:"}</p>
                            <input
                                type="text"
                                value={self.connect_string.clone()}
                                oninput={on_connect_input}
                                placeholder="Enter address or peer id..."
                            />
                            <button class="streuen-settings-bubble" onclick={connect_to_peer_click}>{ "Connect" }</button>
                        </form>
                    </div>
                </div>
            </>
        }
    }
}
