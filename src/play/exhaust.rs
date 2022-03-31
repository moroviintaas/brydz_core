use serde::{Deserialize, Serialize};
use crate::card::suit::{Suit, SUITS};
use crate::player::side::{Side, SIDES};


#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exhaust{
    array: u64
}
/*
impl Exhaust{
    pub fn new() -> Self{
        Self{array:0}
    }

    pub fn get_exhaust(&self, side: Side, suit: Suit) -> bool{
        self.get_suit_exhaust(suit)[side.index()]
    }

}*/

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExhaustTable {
    spades: [bool; SIDES.len()],
    hearts: [bool; SIDES.len()],
    diamonds: [bool; SIDES.len()],
    clubs: [bool; SIDES.len()],

}

impl ExhaustTable {
    pub fn new() -> Self{
        ExhaustTable {spades: [false;SIDES.len()], hearts: [false;SIDES.len()], diamonds: [false;SIDES.len()], clubs: [false;SIDES.len()]}
    }
    pub fn get_suit_exhaust(&self, suit: Suit) -> [bool; SIDES.len()]{
        match suit{
            Suit::Spades => self.spades,
            Suit::Hearts => self.hearts,
            Suit::Diamonds => self.diamonds,
            Suit::Clubs => self.clubs
        }
    }
    pub fn get_side_exhaust(&self, side: Side) -> [bool; SUITS.len()]{
        /*match side{
            Side::North => [self.spades[], self.hearts[0], self.diamonds[0], self.clubs[0]],
            Side::East => [self.spades[1], self.hearts[1], self.diamonds[1], self.clubs[1]],
            Side::South => [self.spades[2], self.hearts[2], self.diamonds[2], self.clubs[2]],
            Side::West => [self.spades[3], self.hearts[3], self.diamonds[3], self.clubs[3]],

        }*/
        [self.spades[side.index()], self.hearts[side.index()], self.diamonds[side.index()], self.clubs[side.index()]]

    }
    pub fn get_exhaust(&self, side: Side, suit: Suit) -> bool{
        self.get_suit_exhaust(suit)[side.index()]
    }
    /// Marks suit as exhausted for a side
    ///
    /// # Examples:
    /// ```
    /// use bridge_core::play::exhaust::ExhaustTable;
    /// use bridge_core::card::suit::Suit;
    /// use bridge_core::player::side::Side;
    /// let mut exhaust = ExhaustTable::new();
    /// assert_eq!(exhaust.get_exhaust(Side::North, Suit::Spades), false);
    /// exhaust.exhaust(&Side::North, &Suit::Spades);
    /// assert_eq!(exhaust.get_exhaust(Side::North, Suit::Spades), true);
    /// ```
    ///
    pub fn exhaust(&mut self, side: &Side, suit: &Suit){
        match suit{
            Suit::Spades => self.spades[side.index()] = true,
            Suit::Hearts => self.hearts[side.index()] = true,
            Suit::Diamonds => self.diamonds[side.index()] = true,
            Suit::Clubs => self.clubs[side.index()] = true
        };
    }


}

impl Default for ExhaustTable {
    fn default() -> Self {
        Self::new()
    }
}