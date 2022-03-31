use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::card::suit::Suit::{Clubs, Diamonds, Hearts, Spades};

///Enum representing suits of card
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Hash)]
pub enum Suit{
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

impl Suit{
    pub fn age(&self) -> u8{
        match self{
            Spades => 3,
            Hearts => 2,
            Diamonds => 1,
            Clubs => 0
        }
    }

}

pub const SUITS: [Suit; 4] = [Spades, Hearts, Diamonds, Clubs];



impl PartialOrd<Self> for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.age().cmp(&other.age()))
    }
}

impl Ord for Suit{
    fn cmp(&self, other: &Self) -> Ordering {
        self.age().cmp(&other.age())
    }
}

#[cfg(test)]
mod tests{
    use crate::card::suit::Suit;

    #[test]
    fn test_order(){
        let spades = Suit::Spades;
        let hearts = Suit::Hearts;
        let diamonds = Suit::Diamonds;
        let clubs = Suit::Clubs;

        assert_eq!( spades, spades);
        assert!(spades > hearts);
        assert!(spades > clubs);
        assert!(hearts > clubs && diamonds > clubs);
        assert!(clubs < spades);
    }
}