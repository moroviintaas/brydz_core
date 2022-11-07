use std::fmt::{Display, Formatter};
use karty::cards::{Card2Sym, CardStd};
use karty::suits::{Suit};
use crate::error::bidding::BiddingError;


use crate::error::contract::ContractError;
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
pub enum BridgeCoreError<Card: Card2Sym>{
    Deal(ContractError<Card>),
    Bidding(BiddingError<Card::Suit>),
    Score(ScoreError),
    Trick(TrickError<Card>),
    Distribution(DistributionError),
    Hand(HandError),
    Format(FormatError),
    Custom(String),


}

impl<Card: Card2Sym> Display for BridgeCoreError<Card> {
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

impl<Card: Card2Sym> std::error::Error for BridgeCoreError<Card>{}





pub type BridgeCoreErrorStd = BridgeCoreError<CardStd>;
/*
impl<F: Figure, S: Suit>  From<std::io::Error> for BridgeError<F, S>{
    fn from(e: std::io::Error) -> Self {
        Self::IO(e.kind())
    }
}
*/