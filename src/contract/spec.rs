//use serde::{Deserialize, Serialize};
use karty::suits::SuitTrait;
use crate::error::BiddingErrorGen::{DoubleAfterDouble, DoubleAfterReDouble, ReDoubleAfterReDouble, ReDoubleWithoutDouble};
use crate::bidding::{Doubling};
use crate::player::side::Side;
use crate::bidding::Bid;
use crate::error::BiddingErrorGen;


#[derive(Debug, Eq, PartialEq,  Clone)]
//#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContractSpec<S: SuitTrait> {
    declarer: Side,
    bid: Bid<S>,
    doubling: Doubling
}

impl<S: SuitTrait> ContractSpec<S> {
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

    pub fn double(&mut self) -> Result<(), BiddingErrorGen<S>>{
        match self.doubling{
            Doubling::None => {
                self.doubling = Doubling::Double;
                Ok(())
            },
            Doubling::Double => Err(DoubleAfterDouble),
            Doubling::ReDouble => Err(DoubleAfterReDouble)
        }
    }

    pub fn redouble(&mut self) -> Result<(), BiddingErrorGen<S>>{
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