mod messages;
mod window;
mod users_panel;
mod navigation;

use libp2p::swarm::Swarm;
use yew::prelude::*;

use crate::chat::users_panel::UsersPanel;
use crate::chat::window::ChatWindow;
use crate::chat::navigation::Navigation;

pub enum ChatMsg {
    SelectUser(String),
    AddUser(String),
    RemoveUser(String),
}

pub struct Chat {
    swarm: Swarm<crate::libp2p::ChatBehavior>,
    users: Vec<String>,
    selected_user: String,
}

impl Component for Chat {
    type Message = ChatMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let swarm = crate::libp2p::build_swarm().expect("Failed to create libp2p swarm.");
        Self {
            swarm: swarm,
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
        let on_select_user =
            ctx.link().callback(ChatMsg::SelectUser);
        let on_add_user =
            ctx.link().callback(ChatMsg::AddUser);
        let on_remove_user =
            ctx.link().callback(ChatMsg::RemoveUser);
        html! {
            <>
                <div style="display: flex; flex-direction: column; height: 100vh; min-height: 0;">
                    <Navigation peer_id={self.swarm.local_peer_id().clone()} />
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
