#[cfg(target_arch = "wasm32")]
mod chat;
#[cfg(target_arch = "wasm32")]
mod libp2p;

#[cfg(target_arch = "wasm32")]
use crate::chat::Chat;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Chat />
        </>
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    tracing_wasm::set_as_global_default();
    yew::Renderer::<App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Not currently implemented for non wasm targets");
}
