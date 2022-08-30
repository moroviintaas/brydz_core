use std::error::Error;
use std::fmt::{Display, Formatter};
use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::bidding::BiddingError;


use crate::error::deal::DealError;
use crate::error::{DistributionError, ScoreError, TrickError};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch<T>{
    pub expected: T,
    pub found: T
}
impl<T: Copy> Copy for Mismatch<T>{}



impl<S:Suit> Display for BiddingError<S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeError<F: Figure, S: Suit>{
    DealError(DealError<F, S>),
    BiddingError(BiddingError<S>),
    Score(ScoreError),
    Trick(TrickError<F, S>),
    Distribution(DistributionError),
    Custom(String),

}

impl<F: Figure, S: Suit> Display for BridgeError<F, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            BridgeError::DealError(deal_error)=> match f.alternate(){
                true => write!(f, "BridgeError::DealError {{ {:#} }} ", deal_error ),
                false => write!(f, "BridgeError::DealError {{ {} }} ", deal_error ),
            }
            _ => {todo!()}
        }

    }
}

impl<F: Figure, S: Suit> Error for BridgeError<F, S>{

}
