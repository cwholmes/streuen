#[cfg(target_arch = "wasm32")]
fn main() {
    tracing_wasm::set_as_global_default();
    yew::Renderer::<streuen_chat_web::app::App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Not currently implemented for non wasm targets");
}
