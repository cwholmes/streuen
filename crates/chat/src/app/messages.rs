pub struct Message {
    message_id: u32,
    to: super::users::User,
    from: super::users::User,
    text: String,
}

pub struct Messages {
    messages: Vec<Message>,
}

impl Messages {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}
