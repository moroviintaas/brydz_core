use std::fmt::{Display, Formatter};
use karty::cards::{Card2Sym, CardStd};
use crate::error::{BridgeCoreError, TrickError};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum ContractError<Card: Card2Sym>{
    DealFull,
    DealIncomplete,
    DuplicateCard(Card),
    TrickError(TrickError<Card>),
    IndexedOverCurrentTrick(usize),
    DummyReplaceAttempt,
    DummyNotPlaced,

}
impl<Card: Card2Sym>Display for ContractError<Card>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type DealErrorStd = ContractError<CardStd>;

impl<Card: Card2Sym> From<ContractError<Card>> for BridgeCoreError<Card>{
    fn from(e: ContractError<Card>) -> Self {
        Self::Deal(e)
    }
}