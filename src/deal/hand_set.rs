use std::{collections::HashSet};

use karty::{symbol::CardSymbol, cards::CardStd};

use crate::{error::HandError};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use super::hand::Hand;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct HandSet<Crd: CardSymbol>{
    cards: HashSet<Crd>,
    //_phantom: PhantomData<>
}

impl <Crd: CardSymbol> IntoIterator for HandSet<Crd>{
    type Item = Crd;

    type IntoIter = std::collections::hash_set::IntoIter<Crd>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<Crd: CardSymbol> Hand for HandSet<Crd>{
    type CardType = Crd;
    fn add_card(&mut self, card: Crd) -> Result<(), crate::error::HandError> {
        if self.cards.insert(card){
            Ok(())
        }
        else{
            Err(HandError::CardDuplicated)
        }
    }

    fn remove_card(&mut self, card: &Crd) -> Result<(), crate::error::HandError> {
        match self.cards.remove(card){
            true => Ok(()),
            false => Err(HandError::CardNotInHand)
        }
    }

    fn new_empty() -> Self {
        Self{cards: HashSet::new()}
    }

    fn contains(&self, card: &Crd) -> bool {
        self.cards.contains(card)
    }
    fn len(&self) -> usize{
        self.cards.len()
    }
}

pub type HandSetStd = HandSet<CardStd>;


impl<Crd: CardSymbol> HandSet<Crd>{
    

}