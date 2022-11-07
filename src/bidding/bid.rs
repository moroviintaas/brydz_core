use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use karty::suits::{Suit, SuitTrait};
use crate::cards::trump::Trump;
use crate::error::BiddingErrorGen;
use crate::error::BiddingErrorGen::IllegalBidNumber;
use crate::meta::{HALF_TRICKS, MAX_BID_NUMBER, MIN_BID_NUMBER};

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Bid<S: SuitTrait> {
    trump: Trump<S>,
    number: u8
}

pub type BidStd = Bid<Suit>;

impl <S: SuitTrait + Copy> Copy for Bid<S>{}

impl<S: SuitTrait>  Bid<S> {
    pub fn init(trump: Trump<S>, number: u8) -> Result<Self, BiddingErrorGen<S>>{
        match number{
            legit @MIN_BID_NUMBER..=MAX_BID_NUMBER => Ok(Self{trump, number: legit}),
            no_legit => Err(IllegalBidNumber(no_legit))

        }
    }
    pub fn trump(&self) -> &Trump<S>{
        &self.trump
    }
    pub fn number(&self) -> u8{
        self.number
    }
    pub fn number_normalised(&self) -> u8{
        self.number + HALF_TRICKS
    }
}
impl<S: SuitTrait> PartialOrd for Bid<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.number.cmp(&other.number).then_with(|| {
            self.trump.cmp(&other.trump)
        }))
    }
}

impl<S: SuitTrait + Display> Display for Bid<S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){

        }
        todo!();
    }
}


/// Delivers `Ord` for `Bid`
/// ```
/// use std::cmp::Ordering;
/// use brydz_core::cards::trump::Trump::{Colored, NoTrump};
/// use karty::suits::Suit::*;
/// use brydz_core::bidding::Bid;
/// let bid1 = Bid::init(NoTrump, 2).unwrap();
/// let bid2 = Bid::init(Colored(Spades), 3).unwrap();
/// let bid3 = Bid::init(Colored(Clubs), 3).unwrap();
/// let bid4 = Bid::init(Colored(Hearts), 4).unwrap();
/// let bid5 = Bid::init(Colored(Diamonds), 2).unwrap();
/// assert!(bid1 < bid2);
/// assert!(bid2 > bid3);
/// assert!(bid2 < bid4);
/// assert!(bid1 > bid5);
/// ```
impl<S: SuitTrait> Ord for Bid<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number).then_with(||{
            self.trump.cmp(&other.trump)
        })


    }
}
pub mod consts {
    use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    use crate::bidding::Bid;
    use crate::cards::trump::Trump;

    pub const BID_C1: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Clubs), number: 1 };
    pub const BID_C2: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Clubs), number: 2 };
    pub const BID_C3: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Clubs), number: 3 };
    pub const BID_C4: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Clubs), number: 4 };
    pub const BID_C5: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Clubs), number: 5 };
    pub const BID_C6: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Clubs), number: 6 };
    pub const BID_C7: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Clubs), number: 7 };

    pub const BID_D1: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Diamonds), number: 1 };
    pub const BID_D2: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Diamonds), number: 2 };
    pub const BID_D3: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Diamonds), number: 3 };
    pub const BID_D4: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Diamonds), number: 4 };
    pub const BID_D5: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Diamonds), number: 5 };
    pub const BID_D6: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Diamonds), number: 6 };
    pub const BID_D7: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Diamonds), number: 7 };

    pub const BID_H1: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Hearts), number: 1 };
    pub const BID_H2: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Hearts), number: 2 };
    pub const BID_H3: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Hearts), number: 3 };
    pub const BID_H4: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Hearts), number: 4 };
    pub const BID_H5: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Hearts), number: 5 };
    pub const BID_H6: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Hearts), number: 6 };
    pub const BID_H7: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Hearts), number: 7 };

    pub const BID_S1: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Spades), number: 1 };
    pub const BID_S2: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Spades), number: 2 };
    pub const BID_S3: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Spades), number: 3 };
    pub const BID_S4: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Spades), number: 4 };
    pub const BID_S5: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Spades), number: 5 };
    pub const BID_S6: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Spades), number: 6 };
    pub const BID_S7: Bid<karty::suits::Suit> = Bid { trump: Trump::Colored(Spades), number: 7 };

    pub const BID_NT1: Bid<karty::suits::Suit> = Bid { trump: Trump::NoTrump, number: 1 };
    pub const BID_NT2: Bid<karty::suits::Suit> = Bid { trump: Trump::NoTrump, number: 2 };
    pub const BID_NT3: Bid<karty::suits::Suit> = Bid { trump: Trump::NoTrump, number: 3 };
    pub const BID_NT4: Bid<karty::suits::Suit> = Bid { trump: Trump::NoTrump, number: 4 };
    pub const BID_NT5: Bid<karty::suits::Suit> = Bid { trump: Trump::NoTrump, number: 5 };
    pub const BID_NT6: Bid<karty::suits::Suit> = Bid { trump: Trump::NoTrump, number: 6 };
    pub const BID_NT7: Bid<karty::suits::Suit> = Bid { trump: Trump::NoTrump, number: 7 };
}