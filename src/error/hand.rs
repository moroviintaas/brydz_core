use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::BridgeError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HandError{
    CardNotInHand,
    EmptyHand,
}

impl<F: Figure, S: Suit> From<HandError> for BridgeError<F, S>{
    fn from(e: HandError) -> Self {
        Self::Hand(e)
    }
}