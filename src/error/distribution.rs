use karty::{ cards::Card2SymTrait};
use crate::error::BridgeCoreErrorGen;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum DistributionError{
    TooFewCards(usize)
}

impl<Card: Card2SymTrait> From<DistributionError> for BridgeCoreErrorGen<Card>{
    fn from(e: DistributionError) -> Self {
        Self::Distribution(e)
    }
}