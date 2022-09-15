use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::{RecvError, SendError};
use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::BridgeError;
use speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]

pub enum CommError{
    SendError,
    TrySendError,
    RecvError,
    TryRecvError
}

impl Display for CommError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CommError{

}

impl<F: Figure, S: Suit> From<CommError> for BridgeError<F, S>{
    fn from(e: CommError) -> Self {
        Self::Comm(e)
    }
}

impl From<RecvError> for CommError{
    fn from(_: RecvError) -> Self {
        Self::RecvError
    }
}
impl<T> From<SendError<T>> for CommError{
    fn from(_: SendError<T>) -> Self {
        Self::SendError
    }
}

#[cfg(feature = "async")]
impl From<tokio::sync::mpsc::error::TryRecvError> for CommError{
    fn from(_: tokio::sync::mpsc::error::TryRecvError) -> Self {
        Self::TryRecvError
    }
}

#[cfg(feature = "async")]
impl<T> From<tokio::sync::mpsc::error::TrySendError<T>> for CommError{
    fn from(_: tokio::sync::mpsc::error::TrySendError<T>) -> Self {
        Self::TrySendError
    }
}

#[cfg(feature = "async")]
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for CommError{
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::SendError
    }
}

#[cfg(feature = "async")]
impl<F:Figure, S:Suit, T> From<tokio::sync::mpsc::error::SendError<T>> for BridgeError<F, S>{
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        CommError::SendError.into()
    }
}

#[cfg(feature = "async")]
impl<F:Figure, S:Suit> From<tokio::sync::mpsc::error::TryRecvError> for BridgeError<F, S>{
    fn from(_: tokio::sync::mpsc::error::TryRecvError) -> Self {
        CommError::TryRecvError.into()
    }
}

#[cfg(feature = "async")]
impl<F:Figure, S:Suit, T> From<tokio::sync::mpsc::error::TrySendError<T>> for BridgeError<F, S>{
    fn from(_: tokio::sync::mpsc::error::TrySendError<T>) -> Self {
        CommError::TrySendError.into()
    }
}