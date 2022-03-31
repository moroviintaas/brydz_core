use std::cmp::Ordering;
use crate::card::trump::Trump;
use crate::error::AuctionError;
use crate::error::AuctionError::{DoubleAfterDouble, DoubleAfterReDouble, IllegalBidNumber, ReDoubleAfterReDouble, ReDoubleWithoutDouble};
use crate::play::auction;
use crate::play::auction::Doubling;
use crate::player::side::Side;
use serde::{Deserialize, Serialize};




#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Bid {
    trump: Trump,
    number: u8
}

impl Bid {
    pub fn create_bid(trump: Trump, number: u8) -> Result<Self, AuctionError>{
        match number{
            legit @1..=7 => Ok(Self{trump, number: legit}),
            no_legit => Err(IllegalBidNumber(no_legit))

        }
    }
    pub fn trump(&self) -> Trump{
        self.trump
    }
}
impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.number.cmp(&other.number).then_with(|| {
            self.trump.cmp(&other.trump)
        }))
    }
}


/// Delivers `Ord` for `Bid`
/// ```
/// use std::cmp::Ordering;
/// use bridge_core::card::trump::Trump::{Colored, NoTrump};
/// use bridge_core::card::suit::*;
/// use bridge_core::card::suit::Suit::{Clubs, Diamonds, Hearts, Spades};
/// use bridge_core::play::contract::Bid;
/// let bid1 = Bid::create_bid(NoTrump, 2).unwrap();
/// let bid2 = Bid::create_bid(Colored(Spades), 3).unwrap();
/// let bid3 = Bid::create_bid(Colored(Clubs), 3).unwrap();
/// let bid4 = Bid::create_bid(Colored(Hearts), 4).unwrap();
/// let bid5 = Bid::create_bid(Colored(Diamonds), 2).unwrap();
/// assert!(bid1 < bid2);
/// assert!(bid2 > bid3);
/// assert!(bid2 < bid4);
/// assert!(bid1 > bid5);
/// ```
impl Ord for Bid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number).then_with(||{
            self.trump.cmp(&other.trump)
        })


    }
}


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Contract {
    owner: Side,
    bid: Bid,
    doubling: auction::Doubling
}

impl Contract {
    pub fn new_d(owner: Side, bid: Bid, doubling: auction::Doubling) -> Self{
        Self{bid, doubling, owner}
    }
    pub fn new(player: Side, bid: Bid) -> Self{
        Self{owner: player, bid, doubling: auction::Doubling::None}
    }
    pub fn bid(&self) -> Bid{
        self.bid
    }
    pub fn doubling(&self) -> Doubling{
        self.doubling
    }
    pub fn owner(&self) -> Side{
        self.owner
    }

    pub fn double(&mut self) -> Result<(), AuctionError>{
        match self.doubling{
            Doubling::None => {
                self.doubling = Doubling::Double;
                Ok(())
            },
            Doubling::Double => Err(DoubleAfterDouble),
            Doubling::ReDouble => Err(DoubleAfterReDouble)
        }
    }

    pub fn redouble(&mut self) -> Result<(), AuctionError>{
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