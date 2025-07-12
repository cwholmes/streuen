use yew::prelude::*;
use web_sys::console;
use libp2p::PeerId;

const NAVIGATION_CSS: &str = r#"
.decentral-text-navigation {
    background: #23272a;
    color: #fff;
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 1rem;
    border-bottom: 1px solid #2c2f33;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}
.decentral-text-nav-brand {
    font-size: 1.2em;
    font-weight: 700;
    color: #fff;
    text-decoration: none;
}
.decentral-text-user-dropdown {
    position: relative;
    display: inline-block;
}
.decentral-text-user-button {
    background: #36393f;
    color: #fff;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9em;
    transition: background 0.15s;
}
.decentral-text-user-button:hover {
    background: #40444b;
}
.decentral-text-user-dropdown-content {
    display: none;
    position: absolute;
    right: 0;
    background: #36393f;
    min-width: 160px;
    box-shadow: 0 8px 16px rgba(0,0,0,0.2);
    border-radius: 0.5rem;
    z-index: 1000;
    margin-top: 0.25rem;
}
.decentral-text-user-dropdown-content.show {
    display: block;
}
.decentral-text-user-dropdown-item {
    color: #b9bbbe;
    padding: 0.75rem 1rem;
    text-decoration: none;
    display: block;
    transition: background 0.15s;
}
.decentral-text-user-dropdown-item:hover {
    background: #40444b;
    color: #fff;
}
.decentral-text-user-dropdown-item:first-child {
    border-radius: 0.5rem 0.5rem 0 0;
}
.decentral-text-user-dropdown-item:last-child {
    border-radius: 0 0 0.5rem 0.5rem;
}
.decentral-text-user-avatar {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    background: #a04000;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.8em;
    font-weight: 600;
}
"#;

pub enum NavigationMsg {
    ToggleDropdown,
    CloseDropdown,
    CopyPeerIdToClipboard,
}

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub peer_id: PeerId,
}

pub struct Navigation {
    dropdown_open: bool,
}

impl Component for Navigation {
    type Message = NavigationMsg;
    type Properties = NavigationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dropdown_open: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavigationMsg::ToggleDropdown => {
                console::log_1(&format!("Drop Down Selected: {}", ctx.props().peer_id.to_string()).into());
                self.dropdown_open = !self.dropdown_open;
                true
            }
            NavigationMsg::CloseDropdown => {
                console::log_1(&format!("Drop Down Closed: {}", ctx.props().peer_id.to_string()).into());
                self.dropdown_open = false;
                true
            }
            NavigationMsg::CopyPeerIdToClipboard => {
                let clipboard = web_sys::window().expect("global window does not exist").navigator().clipboard();
                clipboard.write_text(&ctx.props().peer_id.to_string());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_toggle_dropdown = ctx.link().callback(|_| NavigationMsg::ToggleDropdown);
        // let on_close_dropdown = ctx.link().callback(|_| NavigationMsg::CloseDropdown);
        // let on_nav_click = ctx.link().callback(|e: MouseEvent| {
        //     e.stop_propagation();
        //     NavigationMsg::CloseDropdown
        // });

        let copy_peer_id_to_clipboard = ctx.link().callback(|_| NavigationMsg::CopyPeerIdToClipboard);

        // Get first letter of username for avatar
        let avatar_letter = ctx.props().peer_id.to_string().chars().next().unwrap_or('U').to_uppercase().to_string();

        html! {
            <>
                <style>{ NAVIGATION_CSS }</style>
                <div class="decentral-text-navigation">
                    <a href="#" class="decentral-text-nav-brand">
                        { "Decentral Text" }
                    </a>
                    <div class="decentral-text-user-dropdown">
                        <button 
                            class="decentral-text-user-button" 
                            onclick={on_toggle_dropdown}
                        >
                            <div class="decentral-text-user-avatar">
                                { avatar_letter }
                            </div>
                            <span>{ "Me" }</span>
                            <span>{ "▼" }</span>
                        </button>
                        <div class={classes!(
                            "decentral-text-user-dropdown-content",
                            if self.dropdown_open { Some("show") } else { None }
                        )}>
                            <a href="#" class="decentral-text-user-dropdown-item" onclick={copy_peer_id_to_clipboard}>
                                { "Copy Peer Id" }
                            </a>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
