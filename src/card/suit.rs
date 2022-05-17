use std::cmp::Ordering;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::card::suit::SuitStd::{Clubs, Diamonds, Hearts, Spades};


pub trait Suit: Debug + Eq + Ord + Clone{
    const NUMBER_OF_SUITS: u8;

}
///Enum representing suits of card
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Hash)]
pub enum SuitStd {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

impl SuitStd {
    pub fn age(&self) -> u8{
        match self{
            Spades => 3,
            Hearts => 2,
            Diamonds => 1,
            Clubs => 0
        }
    }

}

pub const SUITS: [SuitStd; 4] = [Spades, Hearts, Diamonds, Clubs];



impl PartialOrd<Self> for SuitStd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.age().cmp(&other.age()))
    }
}

impl Ord for SuitStd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.age().cmp(&other.age())
    }
}

impl Suit for SuitStd{ const NUMBER_OF_SUITS: u8 = 4; }

#[cfg(test)]
mod tests{
    use crate::card::suit::SuitStd;

    #[test]
    fn test_order(){
        let spades = SuitStd::Spades;
        let hearts = SuitStd::Hearts;
        let diamonds = SuitStd::Diamonds;
        let clubs = SuitStd::Clubs;

        assert_eq!( spades, spades);
        assert!(spades > hearts);
        assert!(spades > clubs);
        assert!(hearts > clubs && diamonds > clubs);
        assert!(clubs < spades);
    }
}