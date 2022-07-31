use karty::suits::Suit;
use crate::error::BiddingError;
use crate::error::BiddingError::{DoubleAfterDouble, DoubleAfterReDouble, ReDoubleAfterReDouble, ReDoubleWithoutDouble};
use crate::bidding::{call};
use crate::bidding::call::{Doubling};
use crate::player::side::Side;
use crate::bidding::bid::Bid;


#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct Contract<S: Suit> {
    declarer: Side,
    bid: Bid<S>,
    doubling: call::Doubling
}

impl<S: Suit> Contract<S> {
    pub fn new_d(owner: Side, bid: Bid<S>, doubling: call::Doubling) -> Self{
        Self{bid, doubling, declarer: owner }
    }
    pub fn new(player: Side, bid: Bid<S>) -> Self{
        Self{ declarer: player, bid, doubling: call::Doubling::None}
    }
    pub fn bid(&self) -> &Bid<S>{
        &self.bid
    }
    pub fn doubling(&self) -> Doubling{
        self.doubling
    }
    pub fn declarer(&self) -> Side{
        self.declarer
    }

    pub fn double(&mut self) -> Result<(), BiddingError<S>>{
        match self.doubling{
            Doubling::None => {
                self.doubling = Doubling::Double;
                Ok(())
            },
            Doubling::Double => Err(DoubleAfterDouble),
            Doubling::ReDouble => Err(DoubleAfterReDouble)
        }
    }

    pub fn redouble(&mut self) -> Result<(), BiddingError<S>>{
        match self.doubling{
            Doubling::Double => {
                self.doubling = Doubling::ReDouble;
                Ok(())
            },
            Doubling::ReDouble => Err(ReDoubleAfterReDouble),
            Doubling::None => Err(ReDoubleWithoutDouble)
        }
    }

}