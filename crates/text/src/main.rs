mod chat_messages;
mod chat_window;
mod libp2p;
mod users_panel;

use crate::chat_window::ChatWindow;
use crate::users_panel::UsersPanel;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let users = use_state(|| vec!["alice".to_string(), "bob".to_string(), "me".to_string()]);
    let selected_user = use_state(|| "alice".to_string());

    // Callbacks for selecting, adding, and removing users
    let on_select_user = {
        let selected_user = selected_user.clone();
        Callback::from(move |user: String| selected_user.set(user))
    };
    let on_add_user = {
        let users = users.clone();
        Callback::from(move |user: String| {
            let mut new_users = (*users).clone();
            if !new_users.contains(&user) {
                new_users.push(user);
                users.set(new_users);
            }
        })
    };
    let on_remove_user = {
        let users = users.clone();
        let selected_user = selected_user.clone();
        Callback::from(move |user: String| {
            let mut new_users = (*users).clone();
            if let Some(pos) = new_users.iter().position(|u| u == &user) {
                new_users.remove(pos);
                // If the removed user is the selected user, select the first user if any
                if *selected_user == user {
                    if let Some(first) = new_users.first() {
                        selected_user.set(first.clone());
                    } else {
                        selected_user.set(String::new());
                    }
                }
                users.set(new_users);
            }
        })
    };

    html! {
        <>
            <div style="display: flex; height: 100vh;">
                <div style="width: 220px; border-right: 1px solid #23272a;">
                    <UsersPanel
                        users={(*users).clone()}
                        selected_user={(*selected_user).clone()}
                        on_select_user={on_select_user}
                        on_add_user={on_add_user}
                        on_remove_user={on_remove_user}
                    />
                </div>
                <div style="flex: 1; display: flex; justify-content: center; align-items: flex-start; padding: 2rem;">
                    <ChatWindow selected_user={(*selected_user).clone()} />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
