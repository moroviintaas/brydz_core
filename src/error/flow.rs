use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::{RecvError, SendError, TryRecvError};
use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::BridgeError;
use crate::player::side::Side;
use crate::protocol::{ClientMessage, ServerMessage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowError{
    ServerDead,
    AbsentPlayer(Side),
    ImpersonationAbuse,
    UnexpectedServerMessage(Box<ServerMessage>),
    UnexpectedClientMessage(Box<ClientMessage>),
    PlayAfterEnd(Side),
    ConfusingMessage,
    RecvError,
    TryRecvError,
    SendError,
    MissingConnection(Side),
    DifferentSideExpected(Side)


}

impl<F: Figure, S: Suit> From<FlowError> for BridgeError<F, S>{
    fn from(e: FlowError) -> Self {
        Self::Flow(e)
    }
}

impl Display for FlowError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for FlowError{}

impl<F:Figure, S:Suit, T > From<SendError<T>> for BridgeError<F, S>{
    fn from(_: SendError<T>) -> Self {
        Self::Flow(FlowError::SendError)
    }
}

impl<F:Figure, S:Suit> From<RecvError> for BridgeError<F, S>{
    fn from(_: RecvError) -> Self {
        Self::Flow(FlowError::RecvError)
    }
}
impl<F:Figure, S:Suit> From<TryRecvError> for BridgeError<F, S>{
    fn from(_: TryRecvError) -> Self {
        Self::Flow(FlowError::TryRecvError)
    }
}

impl From<RecvError> for FlowError{
    fn from(_: RecvError) -> Self {
        Self::RecvError
    }
}
impl<T> From<SendError<T>> for FlowError{
    fn from(_: SendError<T>) -> Self {
        Self::SendError
    }
}