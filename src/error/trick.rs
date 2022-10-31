use std::fmt::{Display, Formatter};
use karty::cards::{ Card2Sym};
use crate::error::{BridgeCoreError, Mismatch};
use crate::player::side::Side;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Eq, PartialEq,  Clone)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum TrickError<Card: Card2Sym>{
    MissingCard(Side),
    CardSlotAlreadyUsed(Side),
    DuplicateCard(Card),
    ViolatedOrder(Mismatch<Side>),

    UsedPreviouslyExhaustedSuit(Card::Suit),
    TrickFull,
}
impl<Card: Card2Sym> Display for TrickError<Card> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl<Card: Card2Sym> From<TrickError<Card>> for BridgeCoreError<Card>{
    fn from(e: TrickError<Card>) -> Self {
        Self::Trick(e)
    }
}