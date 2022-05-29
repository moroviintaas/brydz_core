use std::cmp::Ordering;
use carden::suits::Suit;
use carden::suits::SuitStd::{Spades, Diamonds, Hearts, Clubs};
use crate::play::trump::Trump;
use crate::error::AuctionError;
use crate::error::AuctionError::IllegalBidNumber;
pub const MIN_BID_NUMBER: u8 = 1;
pub const MAX_BID_NUMBER: u8 = 7;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Bid<S: Suit> {
    trump: Trump<S>,
    number: u8
}
impl <S: Suit + Copy> Copy for Bid<S>{}

impl<S: Suit>  Bid<S> {
    pub fn create_bid(trump: Trump<S>, number: u8) -> Result<Self, AuctionError<S>>{
        match number{
            legit @MIN_BID_NUMBER..=MAX_BID_NUMBER => Ok(Self{trump, number: legit}),
            no_legit => Err(IllegalBidNumber(no_legit))

        }
    }
    pub fn trump(&self) -> &Trump<S>{
        &self.trump
    }
}
impl<S: Suit> PartialOrd for Bid<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.number.cmp(&other.number).then_with(|| {
            self.trump.cmp(&other.trump)
        }))
    }
}


/// Delivers `Ord` for `Bid`
/// ```
/// use std::cmp::Ordering;
/// use bridge_core::play::trump::Trump::{Colored, NoTrump};
/// use carden::suits::standard::SuitStd::*;
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
impl<S: Suit> Ord for Bid<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number).then_with(||{
            self.trump.cmp(&other.trump)
        })


    }
}

pub const BID_C1: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Clubs), number: 1};
pub const BID_C2: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Clubs), number: 2};
pub const BID_C3: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Clubs), number: 3};
pub const BID_C4: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Clubs), number: 4};
pub const BID_C5: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Clubs), number: 5};
pub const BID_C6: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Clubs), number: 6};
pub const BID_C7: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Clubs), number: 7};

pub const BID_D1: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Diamonds), number: 1};
pub const BID_D2: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Diamonds), number: 2};
pub const BID_D3: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Diamonds), number: 3};
pub const BID_D4: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Diamonds), number: 4};
pub const BID_D5: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Diamonds), number: 5};
pub const BID_D6: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Diamonds), number: 6};
pub const BID_D7: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Diamonds), number: 7};

pub const BID_H1: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Hearts), number: 1};
pub const BID_H2: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Hearts), number: 2};
pub const BID_H3: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Hearts), number: 3};
pub const BID_H4: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Hearts), number: 4};
pub const BID_H5: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Hearts), number: 5};
pub const BID_H6: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Hearts), number: 6};
pub const BID_H7: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Hearts), number: 7};

pub const BID_S1: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Spades), number: 1};
pub const BID_S2: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Spades), number: 2};
pub const BID_S3: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Spades), number: 3};
pub const BID_S4: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Spades), number: 4};
pub const BID_S5: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Spades), number: 5};
pub const BID_S6: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Spades), number: 6};
pub const BID_S7: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::Colored(Spades), number: 7};

pub const BID_NT1: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::NoTrump, number: 1};
pub const BID_NT2: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::NoTrump, number: 2};
pub const BID_NT3: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::NoTrump, number: 3};
pub const BID_NT4: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::NoTrump, number: 4};
pub const BID_NT5: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::NoTrump, number: 5};
pub const BID_NT6: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::NoTrump, number: 6};
pub const BID_NT7: Bid<carden::suits::standard::SuitStd> = Bid{trump: Trump::NoTrump, number: 7};