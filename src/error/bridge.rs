use std::error::Error;
use std::fmt::{Display, Formatter};
use karty::figures::{Figure, FigureStd};
use karty::suits::{Suit, SuitStd};
use crate::error::bidding::BiddingError;


use crate::error::deal::DealError;
use crate::error::{DistributionError, HandError, ScoreError, TrickError};

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use crate::error::FormatError;




#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
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
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum BridgeCoreError<F: Figure, S: Suit>{
    Deal(DealError<F, S>),
    Bidding(BiddingError<S>),
    Score(ScoreError),
    Trick(TrickError<F, S>),
    Distribution(DistributionError),
    Hand(HandError),
    Format(FormatError),
    Custom(String),


}

impl<F: Figure, S: Suit> Display for BridgeCoreError<F, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            BridgeCoreError::Deal(deal_error)=> match f.alternate(){
                true => write!(f, "BridgeError::DealError {{ {:#} }} ", deal_error ),
                false => write!(f, "BridgeError::DealError {{ {} }} ", deal_error ),
            }
            _ => {todo!()}
        }

    }
}

impl<F: Figure, S: Suit> Error for BridgeCoreError<F, S>{

}


pub type BridgeCoreErrorStd = BridgeCoreError<FigureStd, SuitStd>;
/*
impl<F: Figure, S: Suit>  From<std::io::Error> for BridgeError<F, S>{
    fn from(e: std::io::Error) -> Self {
        Self::IO(e.kind())
    }
}
*/