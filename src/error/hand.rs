use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::BridgeCoreError;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum HandError{
    CardNotInHand,
    EmptyHand,
    HandFull,
    CardDuplicated
}

impl<F: Figure, S: Suit> From<HandError> for BridgeCoreError<F, S>{
    fn from(e: HandError) -> Self {
        Self::Hand(e)
    }
}