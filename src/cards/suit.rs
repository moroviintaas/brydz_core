use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::cards::suit::Suit::{Clubs, Diamonds, Hearts, Spades};

///Enum representing suits of cards
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Hash)]
pub enum Suit{
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

impl Suit{
    fn ord_num(&self) -> u8{
        match self{
            Spades => 4,
            Hearts => 3,
            Diamonds => 2,
            Clubs => 1
        }
    }

}

pub const SUITS: [Suit; 4] = [Spades, Hearts, Diamonds, Clubs];



impl PartialOrd<Self> for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.ord_num().cmp(&other.ord_num()))
    }
}

impl Ord for Suit{
    fn cmp(&self, other: &Self) -> Ordering {
        self.ord_num().cmp(&other.ord_num())
    }
}

#[cfg(test)]
mod tests{
    use crate::cards::suit::Suit;

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