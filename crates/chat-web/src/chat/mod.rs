mod messages;
mod navigation;
mod users_panel;
mod window;

use web_sys::console;
use yew::prelude::*;

use crate::chat::navigation::Navigation;
use crate::chat::users_panel::UsersPanel;
use crate::chat::window::ChatWindow;

pub enum ChatMsg {
    SelectUser(String),
    AddUser(String),
    RemoveUser(String),
}

pub struct Chat {
    // swarm: Swarm<ChatBehavior>,
    id_keys: libp2p::identity::Keypair,
    users: Vec<String>,
    selected_user: String,
}

impl Component for Chat {
    type Message = ChatMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let keypair = libp2p::identity::Keypair::generate_ed25519();

        console::log_1(&format!("local_id = {}", keypair.public().to_peer_id().to_base58()).into());

        crate::libp2p::build_websys_swarm(&keypair).expect("Failed to create libp2p swarm.");

        Self {
            // swarm: swarm,
            id_keys: keypair,
            users: vec!["me".to_string(), "alice".to_string()],
            selected_user: "me".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChatMsg::SelectUser(user) => {
                self.selected_user = user.clone();
                true
            }
            ChatMsg::AddUser(user) => {
                // let user_peer_id = libp2p::PeerId::from_str(&user).expect("Invalid Peer id.");
                // let dial_result = self.swarm.dial(user_peer_id);
                // match dial_result {
                //     Ok(()) => web_sys::console::log_1(&"Dial successful!".into()),
                //     Err(err) => web_sys::console::log_1(&format!("Dial error: {}", err).into()),
                // }

                // let topic = libp2p::gossipsub::IdentTopic::new("test-decentral-management");

                // let publish_result = self.swarm.behaviour_mut().gossipsub.publish(topic, "Hello".as_bytes());
                // match publish_result {
                //     Ok(message_id) => console::log_1(&format!("publish succeeded: {message_id}").into()),
                //     Err(err) => console::log_1(&format!("publish failed: {err}").into()),
                // }

                // for (peer_id, _) in self.swarm.behaviour().gossipsub.all_peers() {
                //     console::log_1(&format!("Peer connected to topic: {}", peer_id).into());
                // }

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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Callbacks for selecting, adding, and removing users
        let on_select_user = ctx.link().callback(ChatMsg::SelectUser);
        let on_add_user = ctx.link().callback(ChatMsg::AddUser);
        let on_remove_user = ctx.link().callback(ChatMsg::RemoveUser);
        html! {
            <>
                <div style="display: flex; flex-direction: column; height: 100vh; min-height: 0;">
                    <Navigation peer_id={self.id_keys.public().to_peer_id().clone()} />
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
                            <ChatWindow selected_user={self.selected_user.clone()} />
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
