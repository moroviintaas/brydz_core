use std::cmp::Ordering;
use crate::card::trump::Trump;
use crate::error::AuctionError;
use crate::error::AuctionError::IllegalBidNumber;
pub const MIN_BID_NUMBER: u8 = 1;
pub const MAX_BID_NUMBER: u8 = 7;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Bid {
    trump: Trump,
    number: u8
}

impl Bid {
    pub fn create_bid(trump: Trump, number: u8) -> Result<Self, AuctionError>{
        match number{
            legit @MIN_BID_NUMBER..=MAX_BID_NUMBER => Ok(Self{trump, number: legit}),
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
/// use bridge_core::auction::bid::Bid;
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