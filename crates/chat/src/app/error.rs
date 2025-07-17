use std::fmt;

use futures_channel::mpsc;

#[derive(Debug)]
pub enum ChatAppError {
    ChatError,
    ChannelClosed,
    SenderError(futures_channel::mpsc::SendError),
}

impl std::error::Error for ChatAppError {}

impl fmt::Display for ChatAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatAppError::ChatError => {
                write!(f, "ChatError")
            }
            ChatAppError::ChannelClosed => {
                write!(f, "ChannelClosed")
            }
            ChatAppError::SenderError(err) => {
                write!(f, "SenderError({err})")
            }
        }
    }
}

impl<T> From<mpsc::TrySendError<T>> for ChatAppError {
    fn from(err: mpsc::TrySendError<T>) -> Self {
        ChatAppError::SenderError(err.into_send_error())
    }
}
