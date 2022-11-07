use karty::cards::Card2SymTrait;
use karty::suits::{SuitTrait, Suit};
use crate::bidding::Bid;
use crate::error::bridge::Mismatch;
use crate::error::BridgeCoreError;
use crate::player::side::Side;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum BiddingError<S: SuitTrait>{
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

pub type BiddingErrorStd = BiddingError<Suit>;

impl<Card: Card2SymTrait> From<BiddingError<Card::Suit>> for BridgeCoreError<Card>{
    fn from(e: BiddingError<Card::Suit>) -> Self {
        Self::Bidding(e)
    }
}