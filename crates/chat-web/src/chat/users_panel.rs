use web_sys::HtmlInputElement;
use web_sys::console;
use yew::prelude::*;

const USER_PANEL_CSS: &str = r#"
.decentral-text-sidebar {
    background: #23272a;
    color: #fff;
    height: calc(100vh - 60px);
    min-width: 220px;
    max-width: 260px;
    display: flex;
    flex-direction: column;
    box-shadow: 2px 0 8px rgba(0,0,0,0.08);
    overflow: hidden;
}
.decentral-text-sidebar h3 {
    font-size: 1.1em;
    font-weight: 700;
    letter-spacing: 1px;
    margin: 0 0 1em 0;
    padding: 1.2em 1em 0 1em;
    color: #b9bbbe;
}
.decentral-text-user-list {
    flex: 1;
    overflow-y: auto;
    padding: 0 0.5em 0 0.5em;
}
li.user-list-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5em 1em;
    margin-bottom: 0.25em;
    border-radius: 0.5em;
    cursor: pointer;
    background: transparent;
    color: #b9bbbe;
    font-weight: 500;
    position: relative;
    transition: background 0.15s, color 0.15s;
}
li.user-list-item.selected {
    background: #36393f;
    color: #fff;
}
li.user-list-item:hover {
    background: #2c2f33;
    color: #fff;
}
.user-list-item-remove {
    opacity: 0;
    transition: opacity 0.2s;
}
li.user-list-item:hover .user-list-item-remove {
    opacity: 1;
}
.decentral-text-add-user-form {
    display: flex;
    gap: 0.5em;
    padding: 0.75em 0 1em 0;
    border-top: 1px solid #23272a;
    background: #2c2f33;
    margin: 0;
    box-sizing: border-box;
    width: 100%;
}
.decentral-text-add-user-form input {
    width: 100%;
    box-sizing: border-box;
    padding: 0.5em;
    border-radius: 0.5em;
    border: none;
    background: #23272a;
    color: #fff;
    font-size: 1em;
}
.decentral-text-add-user-form input::placeholder {
    color: #72767d;
}
"#;

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
                console::log_1(&format!("new user = {}", val).into());
                self.new_user = val;
                true
            }
            UserPanelMsg::AddUser => {
                let name = (*self.new_user).trim().to_string();
                if !name.is_empty() {
                    let on_add_user = ctx.props().on_add_user.clone();
                    on_add_user.emit(name);
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
                <style>{ USER_PANEL_CSS }</style>
                <div class="decentral-text-sidebar">
                    <h3>{ "USERS" }</h3>
                    <ul class="decentral-text-user-list" style="list-style: none; margin: 0;">
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
                    <form class="decentral-text-add-user-form" onsubmit={onsubmit}>
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
