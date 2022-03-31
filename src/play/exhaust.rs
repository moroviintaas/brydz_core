use serde::{Deserialize, Serialize};
use crate::card::Card;
use crate::card::suit::{Suit, SUITS};
use crate::play::trick::Trick;
use crate::player::side::{Side, SIDES};


#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct UsedCards{
    memory: u64
}
impl UsedCards{
    pub fn new() -> Self{
        Self{memory: 0}
    }
    /// Marks card as used
    /// ```
    /// use bridge_core::play::exhaust::UsedCards;
    /// use bridge_core::card;
    /// let mut reg = UsedCards::new();
    /// assert!(!reg.check_card(card::JACK_HEARTS));
    /// reg.mark_used(card::JACK_HEARTS);
    /// assert!(reg.check_card(card::JACK_HEARTS))
    /// ```
    pub fn mark_used(&mut self, card: Card){
        self.memory = self.memory | card.mask();

    }
    /// Checks if card is used.
    pub fn check_card(&self, card:Card) -> bool{
        !matches!(self.memory & card.mask(), 0)
    }
    pub fn mark_used_trick(&mut self, trick: &Trick){
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(c) = trick[s]{
                self.mark_used(c);
            }
        }
    }

    /// Checks if trick contains card that was registered as used in `register`
    /// # Examples:
    /// ```
    /// use bridge_core::play::exhaust::UsedCards;
    /// use bridge_core::card;
    /// use bridge_core::card::QUEEN_HEARTS;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::player::side::Side;
    ///
    /// let mut register = UsedCards::new();
    /// let mut trick = Trick::new(Side::East);
    /// trick.add_card(Side::East, card::JACK_HEARTS).unwrap();
    /// trick.add_card(Side::South, card::QUEEN_HEARTS).unwrap();
    /// trick.add_card(Side::West, card::TEN_CLUBS).unwrap();
    /// trick.add_card(Side::North, card::EIGHT_DIAMONDS).unwrap();
    /// assert_eq!(register.trick_collision(trick), None);
    /// register.mark_used(card::QUEEN_HEARTS);
    /// assert_eq!(register.trick_collision(trick), Some(QUEEN_HEARTS))
    /// ```
    pub fn trick_collision(&self, trick: Trick) -> Option<Card>{
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(c) = trick[s]{
                if self.check_card(c){
                    return Some(c)

                }
            }
        }
        None
    }
}




#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Exhaust{
    array: u16
}

impl Exhaust{
    pub fn new() -> Self{
        Self{array:0}
    }

    pub fn get_exhaust(&self, side: Side, suit: Suit) -> bool{
        self.get_suit_exhaust(suit)[side.index()]
    }

}



#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct SuitExhaust {
    spades: [bool; SIDES.len()],
    hearts: [bool; SIDES.len()],
    diamonds: [bool; SIDES.len()],
    clubs: [bool; SIDES.len()],

}

impl SuitExhaust {
    pub fn new() -> Self{
        SuitExhaust {spades: [false;SIDES.len()], hearts: [false;SIDES.len()], diamonds: [false;SIDES.len()], clubs: [false;SIDES.len()]}
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
    /// use bridge_core::play::exhaust::SuitExhaust;
    /// use bridge_core::card::suit::Suit;
    /// use bridge_core::player::side::Side;
    /// let mut exhaust = SuitExhaust::new();
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

impl Default for SuitExhaust {
    fn default() -> Self {
        Self::new()
    }
}