use karty::suits::Suit;
use crate::error::BiddingError::{DoubleAfterDouble, DoubleAfterReDouble, ReDoubleAfterReDouble, ReDoubleWithoutDouble};
use crate::bidding::{Doubling};
use crate::player::side::Side;
use crate::bidding::Bid;
use crate::error::BiddingError;


#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct ContractSpec<S: Suit> {
    declarer: Side,
    bid: Bid<S>,
    doubling: Doubling
}

impl<S: Suit> ContractSpec<S> {
    pub fn new_d(owner: Side, bid: Bid<S>, doubling: Doubling) -> Self{
        Self{bid, doubling, declarer: owner }
    }
    pub fn new(player: Side, bid: Bid<S>) -> Self{
        Self{ declarer: player, bid, doubling: Doubling::None}
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