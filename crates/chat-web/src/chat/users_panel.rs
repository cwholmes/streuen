use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum UserPanelMsg {
    NewUser(String),
    AddUser,
}

#[derive(Properties, PartialEq)]
pub struct UsersPanelProps {
    pub users: Vec<String>,
    pub selected_user: String,
    pub on_select_user: Callback<String>,
    pub on_add_user: Callback<String>,
    pub on_remove_user: Callback<String>,
}

pub struct UsersPanel {
    new_user: String,
}

impl Component for UsersPanel {
    type Message = UserPanelMsg;
    type Properties = UsersPanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            new_user: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserPanelMsg::NewUser(val) => {
                self.new_user = val;
                true
            }
            UserPanelMsg::AddUser => {
                let name = (*self.new_user).trim().to_string();
                if !name.is_empty() {
                    ctx.props().on_add_user.emit(name);
                    self.new_user = String::new();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: Option<HtmlInputElement> = e.target_dyn_into();
            UserPanelMsg::NewUser(input.map(|i| i.value()).unwrap_or_default())
        });
        let onsubmit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            UserPanelMsg::AddUser
        });

        html! {
            <>
                <div class="streuen-chat-sidebar">
                    <h3>{ "USERS" }</h3>
                    <ul class="streuen-chat-user-list" style="list-style: none; margin: 0;">
                        { for ctx.props().users.iter().map(|user| {
                            let is_selected = *user == ctx.props().selected_user;
                            let on_click = {
                                let user = user.clone();
                                let on_select_user = ctx.props().on_select_user.clone();
                                Callback::from(move |_| on_select_user.emit(user.clone()))
                            };
                            let show_remove = user != "me";
                            let on_remove = {
                                let user = user.clone();
                                let on_remove_user = ctx.props().on_remove_user.clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.stop_propagation();
                                    on_remove_user.emit(user.clone());
                                })
                            };
                            html! {
                                <li
                                    class={classes!("user-list-item", if is_selected { Some("selected") } else { None })}
                                    onclick={on_click}
                                >
                                    <span>{ user }</span>
                                    { if show_remove {
                                        html! {
                                            <button class="user-list-item-remove" onclick={on_remove}>{ "✕" }</button>
                                        }
                                    } else {
                                        html! {}
                                    }}
                                </li>
                            }
                        }) }
                    </ul>
                    <form class="streuen-chat-add-user-form" onsubmit={onsubmit}>
                        <input
                            type="text"
                            value={self.new_user.clone()}
                            oninput={oninput}
                            placeholder="Add user..."
                        />
                    </form>
                </div>
            </>
        }
    }
}
