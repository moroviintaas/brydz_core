use std::fmt::{Debug, Display};

use karty::symbol::CardSymbol;

pub use crate::deal::hand_set::*;
pub use crate::deal::stack_hand::*;
use crate::error::HandError;
pub trait HandTrait: Debug + Clone + Eq + IntoIterator<Item=Self::CardType> + Display{
    type CardType : CardSymbol;
    //type IterType: Iterator<Item=CardSymbol>;
    fn add_card(&mut self, card: Self::CardType) -> Result<(), HandError>;
    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), HandError>;
    fn new_empty() -> Self;
    fn contains(&self, card: &Self::CardType) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool{
        self.len() == 0
    }
    //fn iter(&self) -> Self::IterType;
}




/* 
impl<H: Hand> Default for H
where H::CardType: CardSymbol,{
    fn default() -> Self {
        Self::new_empty()
    }
}*/