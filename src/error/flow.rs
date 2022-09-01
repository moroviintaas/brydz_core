use std::error::Error;
use std::fmt::{Display, Formatter};
use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::BridgeError;
use crate::player::side::Side;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlowError{
    ServerDead,
    PlayerLeft(Side),
    ImpersonationAbuse,
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