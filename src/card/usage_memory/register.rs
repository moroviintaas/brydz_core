use std::fmt::Debug;
use crate::card::Card;
use crate::card::figure::Figure;
use crate::card::suit::Suit;


pub trait CardRegister<F:Figure, S: Suit>: Debug + Default{
    fn mark_used(&mut self, card: &Card<F, S>);
    fn is_card_used(&self, card: &Card<F, S>) -> bool;

}