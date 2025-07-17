mod error;
mod messages;
mod users;

pub struct ChatApp {
    users: users::Users,
    messages: messages::Messages,
}

impl ChatApp {
    pub fn new(name: String) -> Result<Self, error::ChatAppError> {
        let current_user = users::User::new(name);
        Ok(Self {
            users: users::Users::new(current_user),
            messages: messages::Messages::new(),
        })
    }
}
