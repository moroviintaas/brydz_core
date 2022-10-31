use std::fmt::{Display, Formatter};
use karty::cards::{Card2Sym, CardStd};
use crate::error::{BridgeCoreError, TrickError};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum DealError<Card: Card2Sym>{
    DealFull,
    DealIncomplete,
    DuplicateCard(Card),
    TrickError(TrickError<Card>),
    IndexedOverCurrentTrick(usize)

}
impl<Card: Card2Sym>Display for DealError<Card>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type DealErrorStd = DealError<CardStd>;

impl<Card: Card2Sym> From<DealError<Card>> for BridgeCoreError<Card>{
    fn from(e: DealError<Card>) -> Self {
        Self::Deal(e)
    }
}