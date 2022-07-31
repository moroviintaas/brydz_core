use std::fmt::{Display, Formatter};
use karty::figures::Figure;
use karty::suits::Suit;
use crate::bidding::bid::Bid;
use crate::contract::deal::DealError;
use crate::player::side::Side;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch<T>{
    pub expected: T,
    pub found: T
}
impl<T: Copy> Copy for Mismatch<T>{}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiddingError<S: Suit>{
    DoubleAfterDouble,
    DoubleAfterReDouble,
    ReDoubleWithoutDouble,
    ReDoubleAfterReDouble,
    DoubleOnVoidCall,
    ReDoubleOnVoidCall,
    IllegalBidNumber(u8),
    ViolatedOrder(Mismatch<Side>),
    BidTooLow(Mismatch<Bid<S>>),
    DoubleOnSameAxis,
    ReDoubleOnSameAxis

}

impl<S:Suit> Display for BiddingError<S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeError<F: Figure, S: Suit>{
    DealError(DealError<F, S>),
    BiddingError(BiddingError<S>),
    Custom(String)
}