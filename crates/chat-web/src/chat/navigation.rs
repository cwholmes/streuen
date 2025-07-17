use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub on_toggle_settings: Callback<()>,
}

pub struct Navigation {
    pub user_name: String,
}

impl Component for Navigation {
    type Message = ();
    type Properties = NavigationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            user_name: "Me".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get first letter of username for avatar
        let avatar_letter = self
            .user_name
            .to_string()
            .chars()
            .next()
            .unwrap_or('U')
            .to_uppercase()
            .to_string();

        let on_open_settings = ctx.props().on_toggle_settings.clone();
        let open_settings = Callback::from(move |_| on_open_settings.emit(()));

        html! {
            <>
                <div class="streuen-chat-navigation">
                    <a href="#" class="streuen-chat-nav-brand">
                        { "Decentral Text" }
                    </a>
                    <button
                        class="streuen-chat-user-button"
                        onclick={open_settings}
                    >
                        <div class="streuen-chat-user-avatar">
                            { avatar_letter }
                        </div>
                        <span>{ self.user_name.clone() }</span>
                        <span>{ "⚙️" }</span>
                    </button>
                </div>
            </>
        }
    }
}
