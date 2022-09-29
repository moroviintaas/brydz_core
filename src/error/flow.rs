use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::{RecvError, SendError, TryRecvError};
use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::{BridgeError, CommError};
use crate::player::side::Side;
use crate::protocol::{ClientMessage, ServerMessage};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum FlowError{
    ServerDead,
    AbsentPlayer(Side),
    ImpersonationAbuse,
    UnexpectedServerMessage,
    UnexpectedClientMessage,
    PlayAfterEnd(Side),
    ConfusingMessage,
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
        Self::Comm(CommError::SendError)
    }
}

impl<F:Figure, S:Suit> From<RecvError> for BridgeError<F, S>{
    fn from(_: RecvError) -> Self {
        Self::Comm(CommError::RecvError)
    }
}
impl<F:Figure, S:Suit> From<TryRecvError> for BridgeError<F, S>{
    fn from(_: TryRecvError) -> Self {
        Self::Comm(CommError::TryRecvError)
    }
}

