use web_sys::HtmlInputElement;
use web_sys::console;
use yew::prelude::*;

use crate::chat::messages::{ChatMessages, Message};

const CHAT_WINDOW_CSS: &str = r#"
.decentral-text-chat-window {
    background: #36393f;
    color: #fff;
    border-radius: 0.75em;
    box-shadow: 0 2px 8px rgba(0,0,0,0.10);
    display: flex;
    flex-direction: column;
    flex: 1 1 0%;
    min-width: 0;
    height: calc(100vh - 60px - 2rem);
    margin: 1rem;
    overflow: hidden;
    max-width: 800px;
    width: 100%;
}
@media (max-width: 768px) {
    .decentral-text-chat-window {
        margin: 0.5rem;
        height: calc(100vh - 60px - 1rem);
        border-radius: 0.5em;
    }
}
.decentral-text-chat-header {
    padding: 1.2em 1.5em 1em 1.5em;
    font-size: 1.1em;
    font-weight: 700;
    color: #b9bbbe;
    border-bottom: 1px solid #23272a;
    background: #2c2f33;
}
.decentral-text-messages {
    flex: 1;
    overflow-y: auto;
    background: #36393f;
    padding: 1.5em;
    display: flex;
    flex-direction: column;
    gap: 0.75em;
}
.decentral-text-message-row {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
}
.decentral-text-message-row.me {
    align-items: flex-end;
}
.decentral-text-message-sender {
    font-size: 0.8em;
    color: #888;
    margin-bottom: 2px;
}
.decentral-text-message-bubble {
    background: #2c2f33;
    color: #fff;
    padding: 0.7em 1.2em;
    border-radius: 1.2em;
    max-width: 70%;
    word-break: break-word;
    font-size: 1em;
    box-shadow: 0 1px 4px rgba(0,0,0,0.08);
}
.decentral-text-message-row.me .decentral-text-message-bubble {
    background: #d35400;
    color: #fff;
}
.decentral-text-chat-input-form {
    display: flex;
    gap: 0.5em;
    padding: 1em 1.5em 1.2em 1.5em;
    background: #2c2f33;
    border-top: 1px solid #23272a;
}
.decentral-text-chat-input-form input {
    flex: 1;
    padding: 0.7em;
    border-radius: 0.5em;
    border: none;
    background: #23272a;
    color: #fff;
    font-size: 1em;
    box-sizing: border-box;
}
.decentral-text-chat-input-form input::placeholder {
    color: #72767d;
}
.decentral-text-chat-input-form button {
    padding: 0.7em 1.2em;
    border-radius: 0.5em;
    border: none;
    background: #d35400;
    color: #fff;
    font-weight: 600;
    font-size: 1em;
    cursor: pointer;
    transition: background 0.15s;
}
.decentral-text-chat-input-form button:hover {
    background: #a04000;
}
"#;

pub enum ChatWindowMsg {
    UpdateInput(String),
    SendMessage,
}

#[derive(Properties, PartialEq)]
pub struct ChatWindowProps {
    pub selected_user: String,
}

pub struct ChatWindow {
    messages: Vec<Message>,
    input_value: String,
    current_user: String,
}

impl Component for ChatWindow {
    type Message = ChatWindowMsg;
    type Properties = ChatWindowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            messages: vec![Message {
                text: "Welcome to the chat!".to_string(),
                sender: "system".to_string(),
                recipient: "me".to_string(),
            }],
            input_value: String::new(),
            current_user: "me".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChatWindowMsg::UpdateInput(val) => {
                console::log_1(&format!("input = {}", val).into());
                self.input_value = val;
                true
            }
            ChatWindowMsg::SendMessage => {
                if !self.input_value.is_empty() {
                    let selected_user = ctx.props().selected_user.clone();
                    self.messages.push(Message {
                        text: self.input_value.clone(),
                        sender: self.current_user.clone(),
                        recipient: selected_user,
                    });
                    self.input_value.clear();
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
            ChatWindowMsg::UpdateInput(input.map(|i| i.value()).unwrap_or_default())
        });
        let onsubmit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            ChatWindowMsg::SendMessage
        });
        let selected_user = &ctx.props().selected_user;
        let filtered_messages: Vec<Message> = self
            .messages
            .iter()
            .cloned()
            .filter(|msg| {
                (msg.sender == self.current_user && &msg.recipient == selected_user)
                    || (msg.sender == *selected_user && msg.recipient == self.current_user)
                    || (msg.sender == "system" && msg.recipient == self.current_user)
            })
            .collect();
        html! {
            <>
                <style>{ CHAT_WINDOW_CSS }</style>
                <div class="decentral-text-chat-window">
                    <div class="decentral-text-chat-header">{ format!("Chat with {}", selected_user) }</div>
                    <ChatMessages messages={filtered_messages} current_user={self.current_user.clone()} />
                    <form class="decentral-text-chat-input-form" onsubmit={onsubmit}>
                      <input
                          type="text"
                          value={self.input_value.clone()}
                          oninput={oninput}
                          placeholder="Type a message..."
                      />
                      <button type="submit">{ "Send" }</button>
                    </form>
                </div>
            </>
        }
    }
}
