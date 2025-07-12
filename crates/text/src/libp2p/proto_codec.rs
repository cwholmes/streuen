
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
