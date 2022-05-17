use std::fmt::{Display, Formatter};
use carden::suits::Suit;
use crate::auction::bid::Bid;
use crate::player::side::Side;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch<T>{
    pub expected: T,
    pub found: T
}
impl<T: Copy> Copy for Mismatch<T>{}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuctionError<S: Suit>{
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

impl<S:Suit> Display for AuctionError<S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

