use std::fmt::Debug;
use karty::card_element::CardElement;
use karty::cards::Card;
use karty::figures::Figure;
use karty::register::{Register};
use karty::suits::{Suit, SuitStd};
use crate::play::trick::Trick;
use crate::player::side::{Side};

pub trait TrickCollision<F: Figure, S: Suit>{
    fn trick_collision(&self, trick: &Trick<F, S>)->Option<Card<F, S>>;
    fn mark_cards_of_trick(&mut self, trick: &Trick<F, S>);

}

impl <F: Figure, S: Suit, UM> TrickCollision<F, S> for UM
where UM: Register<Card<F,S>>{
    fn trick_collision(&self, trick: &Trick<F, S>) -> Option<Card<F, S>> {
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(card) = &trick[s]{
                if self.is_registered(card){
                    return Some(card.to_owned())

                }
            }
        }
        None
    }

    fn mark_cards_of_trick(&mut self, trick: &Trick<F, S>) {
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(c) = &trick[s]{
                self.register(c.to_owned());
            }
        }
    }
}
#[cfg(test)]
mod tests_card_memory{
    use karty::cards::standard::{EIGHT_DIAMONDS, QUEEN_HEARTS, TEN_CLUBS};
    use karty::register::{Register};
    use karty::register::RegisterCardStd;
    use crate::play::card_trackers::{SuitExhaustStd, TrickCollision};
    use crate::play::trick::Trick;
    use crate::player::side::Side;

    #[test]
    fn trick_collision_std_1(){

        let mut register = RegisterCardStd::default();
        let mut exhaust_register = SuitExhaustStd::default();

        let mut trick = Trick::new(Side::South);
        trick.add_card(Side::South, QUEEN_HEARTS, &mut exhaust_register).unwrap();
        trick.add_card(Side::West, TEN_CLUBS, &mut exhaust_register).unwrap();
        trick.add_card(Side::North, EIGHT_DIAMONDS, &mut exhaust_register).unwrap();
        assert_eq!(register.trick_collision(&trick), None);
        register.register(QUEEN_HEARTS);
        assert_eq!(register.trick_collision(&trick), Some(QUEEN_HEARTS))

    }
}


pub trait SuitExhaustRegister<S: Suit>: Default + Debug{
    fn mark_exhausted(&mut self, side: &Side, suit: &S);
    fn is_exhausted(&self, side: &Side, suit: &S) -> bool;
}
#[derive(Debug, Default)]
pub struct SuitExhaustStd{
    array: u16
}



impl SuitExhaustRegister<SuitStd> for SuitExhaustStd{
    fn mark_exhausted(&mut self, side: &Side, suit: &SuitStd) {
        self.array  |= 1u16 << (usize::from(side.index()*4) + suit.position());
    }

    fn is_exhausted(&self, side: &Side, suit: &SuitStd) -> bool {
        !matches!(self.array & (1u16 << (usize::from(side.index()*4) + suit.position())), 0)
    }
}

