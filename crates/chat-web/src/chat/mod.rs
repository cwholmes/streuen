mod messages;
mod navigation;
mod settings_menu;
mod users_panel;
mod window;

use futures::{SinkExt, StreamExt};
use futures_channel::mpsc;
use streuen_chat::libp2p::{ChatMsgReceive, ChatMsgSend};
use yew::prelude::*;

use crate::chat::navigation::Navigation;
use crate::chat::settings_menu::SettingsMenu;
use crate::chat::users_panel::UsersPanel;
use crate::chat::window::ChatWindow;

pub enum ChatMsg {
    SelectUser(String),
    Bootstrap(libp2p::Multiaddr),
    Connect(libp2p::PeerId),
    AddUser(String),
    RemoveUser(String),
    ToggleSettings,
    Receive(ChatMsgReceive),
}

pub struct Chat {
    id_keys: libp2p::identity::Keypair,
    users: Vec<String>,
    selected_user: String,
    settings_open: bool,
    swarm_sender: mpsc::Sender<ChatMsgSend>,
}

impl Component for Chat {
    type Message = ChatMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let keypair = libp2p::identity::Keypair::generate_ed25519();

        tracing::debug!("local_id = {}", keypair.public().to_peer_id().to_base58());

        let (swarm_sender, mut app_reciever) =
            streuen_chat::libp2p::SwarmStart::new(keypair.clone())
                .start_swarm()
                .unwrap();

        let on_receive_msg = ctx.link().callback(ChatMsg::Receive);

        wasm_bindgen_futures::spawn_local(async move {
            loop {
                if let Some(msg) = app_reciever.next().await {
                    on_receive_msg.emit(msg);
                }
            }
        });

        Self {
            id_keys: keypair,
            users: vec!["me".to_string(), "alice".to_string()],
            selected_user: "me".to_string(),
            settings_open: false,
            swarm_sender,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChatMsg::SelectUser(user) => {
                self.selected_user = user.clone();
                self.settings_open = false;
                true
            }
            ChatMsg::AddUser(user) => {
                if !self.users.contains(&user) {
                    self.users.push(user.clone());
                    self.selected_user = user.clone();
                    true
                } else if self.selected_user != user {
                    self.selected_user = user.clone();
                    true
                } else {
                    false
                }
            }
            ChatMsg::RemoveUser(user) => {
                if let Some(pos) = self.users.iter().position(|u| u == &user) {
                    self.users.remove(pos);
                    if self.selected_user == user {
                        if let Some(first) = self.users.first() {
                            self.selected_user = first.clone();
                        } else {
                            self.selected_user = String::new();
                        }
                    }
                    true
                } else {
                    false
                }
            }
            ChatMsg::ToggleSettings => {
                self.settings_open = !self.settings_open;
                true
            }
            ChatMsg::Bootstrap(addr) => {
                tracing::debug!("Boostrap: {addr}");
                let mut swarm_sender = self.swarm_sender.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    swarm_sender
                        .send(ChatMsgSend::AddBoostrapPeer(addr))
                        .await
                        .unwrap()
                });
                false
            }
            ChatMsg::Connect(peer_id) => {
                tracing::debug!("Connect: {peer_id}");
                let mut swarm_sender = self.swarm_sender.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    swarm_sender
                        .send(ChatMsgSend::Connect(peer_id))
                        .await
                        .unwrap()
                });
                false
            }
            ChatMsg::Receive(msg) => {
                tracing::debug!("Received message in chat app: {msg:?}");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Callbacks for selecting, adding, and removing users
        let on_select_user = ctx.link().callback(ChatMsg::SelectUser);
        let on_add_user = ctx.link().callback(ChatMsg::AddUser);
        let on_remove_user = ctx.link().callback(ChatMsg::RemoveUser);
        let on_toggle_settings = ctx.link().callback(|_| ChatMsg::ToggleSettings);
        let on_boostrap = ctx.link().callback(ChatMsg::Bootstrap);
        let on_connect = ctx.link().callback(ChatMsg::Connect);
        html! {
            <>
                <div style="display: flex; flex-direction: column; height: 100vh; min-height: 0;">
                    <Navigation on_toggle_settings={on_toggle_settings.clone()} />
                    <div style="display: flex; flex: 1; min-height: 0;">
                        <div style="width: 220px; min-width: 220px; border-right: 1px solid #23272a;">
                            <UsersPanel
                                users={self.users.clone()}
                                selected_user={self.selected_user.clone()}
                                on_select_user={on_select_user}
                                on_add_user={on_add_user}
                                on_remove_user={on_remove_user}
                            />
                        </div>
                        <div style="flex: 1; display: flex; justify-content: center; align-items: stretch; min-width: 0;">
                            {
                                if self.settings_open {
                                    html! {
                                        <SettingsMenu
                                            peer_id={self.id_keys.public().to_peer_id().clone()}
                                            on_close={on_toggle_settings.clone()}
                                            bootstrap={on_boostrap}
                                            connect={on_connect}
                                        /> }
                                } else {
                                    html! { <ChatWindow selected_user={self.selected_user.clone()} /> }
                                }
                            }
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
