use karty::figures::Figure;
use karty::suits::{Suit, SuitStd};
use crate::bidding::Bid;
use crate::error::bridge::Mismatch;
use crate::error::BridgeError;
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

pub type BiddingErrorStd = BiddingError<SuitStd>;

impl<F: Figure, S:Suit> From<BiddingError<S>> for BridgeError<F, S>{
    fn from(e: BiddingError<S>) -> Self {
        Self::Bidding(e)
    }
}