use std::fmt::{Display, Formatter};
use karty::cards::Card;
use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::{BridgeError, Mismatch};
use crate::player::side::Side;

#[derive(Debug, Eq, PartialEq,  Clone)]
pub enum TrickError<F: Figure, S: Suit>{
    MissingCard(Side),
    CardSlotAlreadyUsed(Side),
    DuplicateCard(Card<F, S>),
    ViolatedOrder(Mismatch<Side>),

    UsedPreviouslyExhaustedSuit(S),
    TrickFull,
}
impl<F: Figure, S: Suit> Display for TrickError<F, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<F:Figure, S:Suit> From<TrickError<F, S>> for BridgeError<F, S>{
    fn from(e: TrickError<F, S>) -> Self {
        Self::Trick(e)
    }
}