use std::fmt::Debug;
use karty::symbol::CardSymbol;
use karty::cards::{Card2Sym};
use karty::register::{Register};
use karty::suits::{ SuitStd};
use crate::contract::trick::Trick;
use crate::player::side::{Side};

pub trait TrickCollision<Card: Card2Sym>{
    fn trick_collision(&self, trick: &Trick<Card>)->Option<Card>;
    fn mark_cards_of_trick(&mut self, trick: &Trick<Card>);

}

impl <Card: Card2Sym, UM> TrickCollision<Card> for UM
where UM: Register<Card>{
    fn trick_collision(&self, trick: &Trick<Card>) -> Option<Card> {
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(card) = &trick[s]{
                if self.is_registered(card){
                    return Some(card.to_owned())

                }
            }
        }
        None
    }

    fn mark_cards_of_trick(&mut self, trick: &Trick<Card>) {
        for s in [Side::North, Side::East, Side::South, Side::West]{
            if let Some(c) = &trick[s]{
                self.register(c.to_owned());
            }
        }
    }
}
#[cfg(test)]
mod tests_card_memory{
    use karty::cards::{EIGHT_DIAMONDS, QUEEN_HEARTS, TEN_CLUBS};
    use karty::register::{Register};
    use karty::register::RegisterCardStd;
    use crate::contract::collision::{SuitExhaustStd, TrickCollision};
    use crate::contract::trick::Trick;
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



#[derive(Debug, Default, Clone, Copy)]
pub struct SuitExhaustStd{
    array: u16
}




impl Register<(Side, SuitStd)> for SuitExhaustStd{
    fn register(&mut self, element: (Side, SuitStd)) {
        self.array  |= 1u16 << (usize::from(element.0.index()*4) + element.1.position());
    }

    fn unregister(&mut self, element: &(Side, SuitStd)) {
        let mask_neg  = 1u16 << (usize::from(element.0.index()*4) + element.1.position());
        let mask = mask_neg ^ u16::MAX;
        self.array &= mask;
    }

    fn is_registered(&self, element: &(Side, SuitStd)) -> bool {
        !matches!(self.array & (1u16 << (usize::from(element.0.index()*4) + element.1.position())), 0)
    }
}
