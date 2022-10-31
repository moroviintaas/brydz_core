use karty::cards::Card2Sym;
use karty::suits::{Suit, SuitStd};
use crate::bidding::Bid;
use crate::error::bridge::Mismatch;
use crate::error::BridgeCoreError;
use crate::player::side::Side;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
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

impl<Card: Card2Sym> From<BiddingError<Card::Suit>> for BridgeCoreError<Card>{
    fn from(e: BiddingError<Card::Suit>) -> Self {
        Self::Bidding(e)
    }
}