
use karty::cards::Card2SymTrait;

use crate::error::BridgeCoreError;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum HandError{
    CardNotInHand,
    EmptyHand,
    HandFull,
    CardDuplicated,
    HandNotInitialised,
}

impl<Card: Card2SymTrait> From<HandError> for BridgeCoreError<Card>{
    fn from(e: HandError) -> Self {
        Self::Hand(e)
    }
}