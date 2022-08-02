use std::fmt::{Display, Formatter};
use karty::cards::Card;
use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::TrickError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealError<F: Figure, S: Suit>{
    DealFull,
    DealIncomplete,
    DuplicateCard(Card<F, S>),
    TrickError(TrickError<F, S>),
    IndexedOverCurrentTrick(usize)

}
impl<F: Figure, S: Suit>Display for DealError<F, S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}