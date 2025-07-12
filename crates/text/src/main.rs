mod chat;
mod libp2p;

use crate::chat::Chat;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Chat />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
