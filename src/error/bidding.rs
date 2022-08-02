use karty::suits::Suit;
use crate::bidding::Bid;
use crate::error::bridge::Mismatch;
use crate::player::side::Side;

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