use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::BridgeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistributionError{
    TooFewCards(usize)
}

impl<F:Figure, S: Suit> From<DistributionError> for BridgeError<F, S>{
    fn from(e: DistributionError) -> Self {
        Self::Distribution(e)
    }
}