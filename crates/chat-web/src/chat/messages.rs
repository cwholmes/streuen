use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Message {
    pub text: String,
    pub sender: String,
    pub recipient: String,
}

#[derive(Properties, PartialEq)]
pub struct ChatMessagesProps {
    pub messages: Vec<Message>,
    pub current_user: String,
}

#[function_component(ChatMessages)]
pub fn chat_messages(props: &ChatMessagesProps) -> Html {
    html! {
        <div class="streuen-messages">
            {
                props.messages.iter().enumerate().map(|(i, msg)| {
                    let is_me = msg.sender == props.current_user;
                    let show_sender = if i == 0 || props.messages[i-1].sender != msg.sender {
                        true
                    } else {
                        false
                    };
                    html! {
                        <div class={classes!("streuen-message-row", if is_me { Some("me") } else { None })}>
                            { if show_sender {
                                html! { <span class="streuen-message-sender">{ &msg.sender }</span> }
                            } else {
                                html! {}
                            }}
                            <div class="streuen-message-bubble">{ &msg.text }</div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
