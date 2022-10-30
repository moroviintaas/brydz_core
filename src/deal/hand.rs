use karty::symbol::CardSymbol;

pub use crate::deal::hand_vector::*;
pub use crate::deal::hand_set::*;
pub use crate::deal::compact_hand::*;
use crate::error::HandError;
pub trait Hand{
    type CardType : CardSymbol;
    fn add_card(&mut self, card: Self::CardType) -> Result<(), HandError>;
    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), HandError>;
    fn new_empty() -> Self;
    fn contains(&self, card: &Self::CardType) -> bool;
    fn len(&self) -> usize;
}
/* 
impl<H: Hand> Default for H
where H::CardType: CardSymbol,{
    fn default() -> Self {
        Self::new_empty()
    }
}*/