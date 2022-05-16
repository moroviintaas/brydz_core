use std::fmt::{Display, Formatter};
use crate::auction::contract::Bid;
use crate::player::side::Side;
/*
#[derive(Debug, Clone)]
pub enum BridgeError{
    CardSlotAlreadyUsed(Card),
    CardAlreadyUsed(Card),
    PlayerAlreadyPlayed,
    ViolatedPlayOrder,
    PlayerWithoutPlayRole,
    MissingCard(Side),

}

impl Display for BridgeError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}*/

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub struct Mismatch<T: Copy>{
    pub expected: T,
    pub found: T
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuctionError{
    DoubleAfterDouble,
    DoubleAfterReDouble,
    ReDoubleWithoutDouble,
    ReDoubleAfterReDouble,
    DoubleOnVoidCall,
    ReDoubleOnVoidCall,
    IllegalBidNumber(u8),
    ViolatedOrder(Mismatch<Side>),
    BidTooLow(Mismatch<Bid>),
    DoubleOnSameAxis,
    ReDoubleOnSameAxis

}

impl Display for AuctionError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

