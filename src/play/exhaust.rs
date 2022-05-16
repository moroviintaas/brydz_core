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
        !matches!(self.array & (1u16 << ((side.index()*4) + suit.age())), 0)
    }

    /// ```
    /// use bridge_core::play::exhaust::Exhaust;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::card::suit::Suit;
    /// let mut ex_reg = Exhaust::new();
    /// ex_reg.mark_exhausted(Side::East, Suit::Diamonds);
    /// assert_eq!(ex_reg.as_u16(), 0x0020);
    /// ex_reg.mark_exhausted(Side::North, Suit::Clubs);
    /// assert_eq!(ex_reg.as_u16(), 0x0021);
    /// ex_reg.mark_exhausted(Side::South, Suit::Spades);
    /// assert_eq!(ex_reg.as_u16(), 0x0821);
    /// ex_reg.mark_exhausted(Side::West, Suit::Hearts);
    /// assert_eq!(ex_reg.as_u16(), 0x4821);
    /// ```
    pub fn as_u16(&self) -> u16{
        self.array
    }
    ///
    /// ```
    /// use bridge_core::play::exhaust::Exhaust;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::card::suit::Suit;
    /// let mut ex_reg = Exhaust::new();
    /// assert!(!ex_reg.get_exhaust(Side::East, Suit::Diamonds));
    /// ex_reg.mark_exhausted(Side::East, Suit::Diamonds);
    /// assert!(ex_reg.get_exhaust(Side::East, Suit::Diamonds));
    /// ```
    pub fn mark_exhausted(&mut self, side: Side, suit: Suit){
        self.array = self.array | (1u16 << ((side.index()*4) + suit.age()));
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
        [self.spades[usize::from(side.index())], self.hearts[usize::from(side.index())], self.diamonds[usize::from(side.index())], self.clubs[usize::from(side.index())]]

    }
    pub fn get_exhaust(&self, side: Side, suit: Suit) -> bool{
        self.get_suit_exhaust(suit)[usize::from(side.index())]
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
            Suit::Spades => self.spades[usize::from(side.index())] = true,
            Suit::Hearts => self.hearts[usize::from(side.index())] = true,
            Suit::Diamonds => self.diamonds[usize::from(side.index())] = true,
            Suit::Clubs => self.clubs[usize::from(side.index())] = true
        };
    }


}

impl Default for SuitExhaust {
    fn default() -> Self {
        Self::new()
    }
}