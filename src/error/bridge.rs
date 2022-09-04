use std::error::Error;
use std::fmt::{Display, Formatter};
use karty::figures::{Figure, FigureStd};
use karty::suits::{Suit, SuitStd};
use crate::error::bidding::BiddingError;


use crate::error::deal::DealError;
use crate::error::{DistributionError,  HandError, ScoreError, TrickError};
#[cfg(feature="protocol")]
use crate::error::FlowError;



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
    Deal(DealError<F, S>),
    Bidding(BiddingError<S>),
    Score(ScoreError),
    Trick(TrickError<F, S>),
    Distribution(DistributionError),
    #[cfg(feature = "protocol")]
    Flow(FlowError),
    Hand(HandError),
    Custom(String),


}

impl<F: Figure, S: Suit> Display for BridgeError<F, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            BridgeError::Deal(deal_error)=> match f.alternate(){
                true => write!(f, "BridgeError::DealError {{ {:#} }} ", deal_error ),
                false => write!(f, "BridgeError::DealError {{ {} }} ", deal_error ),
            }
            _ => {todo!()}
        }

    }
}

impl<F: Figure, S: Suit> Error for BridgeError<F, S>{

}

pub type BridgeErrorStd = BridgeError<FigureStd, SuitStd>;
