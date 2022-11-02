use std::{collections::HashSet};
use std::fmt::{Display, Formatter};

use karty::{symbol::CardSymbol, cards::CardStd};

use crate::{error::HandError};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use super::hand::Hand;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct HandSet<Crd: CardSymbol + Display>{
    cards: HashSet<Crd>,
    //_phantom: PhantomData<>
}

impl <Crd: CardSymbol + Display> IntoIterator for HandSet<Crd>{
    type Item = Crd;

    type IntoIter = std::collections::hash_set::IntoIter<Crd>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<Crd: CardSymbol + Display> Hand for HandSet<Crd>{
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


impl<Crd: CardSymbol + Display> HandSet<Crd>{
    

}
impl<Crd: CardSymbol + Display> Display for HandSet<Crd>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //let v: Vec<CardStd> = self.cards.iter().collect();
        write!(f,  "[")?;
        if f.alternate(){
            for e in self.cards.iter(){
                write!(f, "{:#}, ", e)?;
            }


        }
        else{
            for e in self.cards.iter(){
                write!(f, "{}, ", e)?;
            }
        }
        write!(f, "]")
    }
}