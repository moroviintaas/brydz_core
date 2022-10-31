use karty::{ cards::Card2Sym};
use crate::error::BridgeCoreError;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum DistributionError{
    TooFewCards(usize)
}

impl<Card: Card2Sym> From<DistributionError> for BridgeCoreError<Card>{
    fn from(e: DistributionError) -> Self {
        Self::Distribution(e)
    }
}