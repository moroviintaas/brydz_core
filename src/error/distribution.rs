use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::BridgeCoreError;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum DistributionError{
    TooFewCards(usize)
}

impl<F:Figure, S: Suit> From<DistributionError> for BridgeCoreError<F, S>{
    fn from(e: DistributionError) -> Self {
        Self::Distribution(e)
    }
}