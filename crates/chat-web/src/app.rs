use streuen_chat::ChatApp;
use streuen_chat::app;
use yew::prelude::*;

use crate::chat::Chat;

pub enum AppMsg {
    RegisterAppHandler(app::AppCallback),
    SwarmDispatchEvent(app::ToChat),
    ChangeUserName(String),
    Receive(app::ToApp),
}

pub struct App {
    chat_app: ChatApp,
}

impl<'a> Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut chat_app = ChatApp::new("Me".to_string()).unwrap();

        tracing::debug!(
            "local_id = {}",
            chat_app.current_user().peer_id().to_base58()
        );

        Self { chat_app }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::RegisterAppHandler(handler) => {
                self.chat_app.register_app_handler(handler);
                false
            }
            AppMsg::SwarmDispatchEvent(event) => {
                self.chat_app.chat_dispatch(event);
                false
            }
            AppMsg::ChangeUserName(_user_name) => true,
            AppMsg::Receive(msg) => {
                tracing::debug!("Received message in chat app: {msg:?}");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let swarm_dispatch = ctx.link().callback(AppMsg::SwarmDispatchEvent);
        let register_app_cb = ctx.link().callback(AppMsg::RegisterAppHandler);
        html! {
            <>
                <Chat
                    peer_id={self.chat_app.current_user().peer_id()}
                    swarm_dispatch_cb={swarm_dispatch.clone()}
                    register_app_cb={register_app_cb.clone()}
                />
            </>
        }
    }
}
