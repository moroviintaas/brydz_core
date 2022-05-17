use std::fmt::Debug;
use crate::card::Card;
use crate::card::figure::Figure;
use crate::card::register::CardRegister;
use crate::card::suit::{SuitStd, Suit};
use crate::play::trick::Trick;
use crate::player::side::{Side};

//pub trait UsedCardRegister<F: Figure>
/*
pub trait UsedCardRegister: Clone + Default + Debug{

}*/
/*
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
    /// assert!(!reg.check_card(&card::JACK_HEARTS));
    /// reg.mark_used(&card::JACK_HEARTS);
    /// assert!(reg.check_card(&card::JACK_HEARTS))
    /// ```
    pub fn mark_used<F: Figure, S: Suit>(&mut self, card: &Card<F, S>){
        //self.memory = self.memory | card.mask();
        todo!();

    }
    /// Checks if card is used.
    pub fn check_card<F: Figure, S: Suit>(&self, card: &Card<F, S>) -> bool{
        //!matches!(self.memory & card.mask(), 0)
        todo!();
    }
    pub fn mark_used_trick<F: Figure, S: Suit>(&mut self, trick: &Trick<F, S>){
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(c) = &trick[s]{
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
    /// assert_eq!(register.trick_collision_legacy(&trick), None);
    /// register.mark_used(&card::QUEEN_HEARTS);
    /// assert_eq!(register.trick_collision_legacy(&trick), Some(QUEEN_HEARTS))
    /// ```
    pub fn trick_collision_legacy<F: Figure, S: Suit>(&self, trick: &Trick<F, S>) -> Option<Card<F, S>>{
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(c) = &trick[s]{
                if self.check_card(c){
                    return Some(c.to_owned())

                }
            }
        }
        None
    }
}
*/
pub trait TrickCollision<F: Figure, S: Suit>{
    fn trick_collision(&self, trick: &Trick<F, S>)->Option<Card<F, S>>;
    fn mark_cards_of_trick(&mut self, trick: &Trick<F, S>);

}

impl <F: Figure, S: Suit, UM> TrickCollision<F, S> for UM
where UM: CardRegister<F,S>{
    fn trick_collision(&self, trick: &Trick<F, S>) -> Option<Card<F, S>> {
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(card) = &trick[s]{
                if self.is_card_used(card){
                    return Some(card.to_owned())

                }
            }
        }
        None
    }

    fn mark_cards_of_trick(&mut self, trick: &Trick<F, S>) {
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(c) = &trick[s]{
                self.mark_used(c);
            }
        }
    }
}
#[cfg(test)]
mod tests_card_memory{
    use crate::card;
    use crate::card::QUEEN_HEARTS;
    use crate::card::register::CardRegister;
    use crate::card::standard_register::CardUsageRegStd;
    use crate::play::card_trackers::{SuitExhaustStd, TrickCollision};
    use crate::play::trick::Trick;
    use crate::player::side::Side;

    #[test]
    fn trick_collision_std_1(){

        let mut register = CardUsageRegStd::default();
        let mut exhaust_register = SuitExhaustStd::default();

        let mut trick = Trick::new(Side::South);
        trick.add_card(Side::South, card::QUEEN_HEARTS, &mut exhaust_register).unwrap();
        trick.add_card(Side::West, card::TEN_CLUBS, &mut exhaust_register).unwrap();
        trick.add_card(Side::North, card::EIGHT_DIAMONDS, &mut exhaust_register).unwrap();
        assert_eq!(register.trick_collision(&trick), None);
        register.mark_used(&card::QUEEN_HEARTS);
        assert_eq!(register.trick_collision(&trick), Some(QUEEN_HEARTS))

    }
}
/*
pub fn trick_collision<F: Figure, S:Suit, UM: CardRegister<F,S>>(register: &R<F, S>, trick: &Trick<F, S>) ->Option<Card<F, S>>{
    for s in [Side::North, Side::East, Side::South, Side::West]{
        if let Some(c) = &trick[s]{
            if register.is_card_used(c){
                    return Some(c.to_owned())

            }
        }
    }
    None
}*/

pub trait SuitExhaustRegister<S: Suit>: Default + Debug{
    fn mark_exhausted(&mut self, side: &Side, suit: &S);
    fn is_exhausted(&self, side: &Side, suit: &S) -> bool;
}
#[derive(Debug)]
pub struct SuitExhaustStd{
    array: u16
}
impl Default for SuitExhaustStd{
    fn default() -> Self {
        Self{array: 0}
    }
}


impl SuitExhaustRegister<SuitStd> for SuitExhaustStd{
    fn mark_exhausted(&mut self, side: &Side, suit: &SuitStd) {
        self.array = self.array | (1u16 << ((side.index()*4) + suit.age()));
    }

    fn is_exhausted(&self, side: &Side, suit: &SuitStd) -> bool {
        !matches!(self.array & (1u16 << ((side.index()*4) + suit.age())), 0)
    }
}

/*
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Exhaust{
    array: u16
}

impl Exhaust{
    pub fn new() -> Self{
        Self{array:0}
    }

    pub fn get_exhaust<S: Suit>(&self, side: Side, suit: &S) -> bool{

        //!matches!(self.array & (1u16 << ((side.index()*4) + suit.age())), 0)
        todo!();
    }

    /// ```
    /// use bridge_core::play::card_trackers::Exhaust;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::card::suit::SuitStd;
    /// let mut ex_reg = Exhaust::new();
    /// ex_reg.mark_exhausted(Side::East, SuitStd::Diamonds);
    /// assert_eq!(ex_reg.as_u16(), 0x0020);
    /// ex_reg.mark_exhausted(Side::North, SuitStd::Clubs);
    /// assert_eq!(ex_reg.as_u16(), 0x0021);
    /// ex_reg.mark_exhausted(Side::South, SuitStd::Spades);
    /// assert_eq!(ex_reg.as_u16(), 0x0821);
    /// ex_reg.mark_exhausted(Side::West, SuitStd::Hearts);
    /// assert_eq!(ex_reg.as_u16(), 0x4821);
    /// ```
    pub fn as_u16(&self) -> u16{
        self.array
    }
    ///
    /// ```
    /// use bridge_core::play::card_trackers::Exhaust;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::card::suit::SuitStd;
    /// let mut ex_reg = Exhaust::new();
    /// assert!(!ex_reg.get_exhaust(Side::East, SuitStd::Diamonds));
    /// ex_reg.mark_exhausted(Side::East, SuitStd::Diamonds);
    /// assert!(ex_reg.get_exhaust(Side::East, SuitStd::Diamonds));
    /// ```
    pub fn mark_exhausted<S: Suit>(&mut self, side: Side, suit: S){
        //self.array = self.array | (1u16 << ((side.index()*4) + suit.age()));
        todo!();
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
    pub fn get_suit_exhaust<S: Suit> (&self, suit: SuitStd) -> [bool; SIDES.len()]{
        match suit{
            SuitStd::Spades => self.spades,
            SuitStd::Hearts => self.hearts,
            SuitStd::Diamonds => self.diamonds,
            SuitStd::Clubs => self.clubs
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
    pub fn get_exhaust<S: Suit> (&self, side: Side, suit: &S) -> bool{
        //self.get_suit_exhaust(suit)[usize::from(side.index())]
        todo!();
    }
    /// Marks suit as exhausted for a side
    ///
    /// # Examples:
    /// ```
    /// use bridge_core::play::card_trackers::SuitExhaust;
    /// use bridge_core::card::suit::SuitStd;
    /// use bridge_core::player::side::Side;
    /// let mut exhaust = SuitExhaust::new();
    /// assert_eq!(exhaust.get_exhaust(Side::North, SuitStd::Spades), false);
    /// exhaust.exhaust(&Side::North, &SuitStd::Spades);
    /// assert_eq!(exhaust.get_exhaust(Side::North, SuitStd::Spades), true);
    /// ```
    ///
    pub fn exhaust<S: Suit> (&mut self, side: &Side, suit: &S){
        /*
        match suit{
            SuitStd::Spades => self.spades[usize::from(side.index())] = true,
            SuitStd::Hearts => self.hearts[usize::from(side.index())] = true,
            SuitStd::Diamonds => self.diamonds[usize::from(side.index())] = true,
            SuitStd::Clubs => self.clubs[usize::from(side.index())] = true
        };
        */

         todo!();
    }


}

impl Default for SuitExhaust {
    fn default() -> Self {
        Self::new()
    }
}

 */