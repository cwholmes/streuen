use std::fmt;

#[derive(Debug)]
pub enum ChatAppError {
    ChatError,
}

impl std::error::Error for ChatAppError {}

impl fmt::Display for ChatAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatAppError::ChatError => {
                write!(f, "ChatAppError")
            }
        }
    }
}
