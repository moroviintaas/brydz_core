use std::fmt::{Display, Formatter};
use karty::cards::Card;

use crate::error::HandError;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use super::hand::Hand;

const LARGEST_MASK:u64 = 1<<63;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct StackHandStd{
    pub(crate) cards: u64,
}

pub struct StackHandStdIterator{
    hand: StackHandStd,
    mask: u64,
    end: bool
}
impl StackHandStdIterator{
    pub fn new(hand: StackHandStd) -> Self{

        Self{mask: 1, hand, end: false}
    }
}
/// ```
/// use brydz_core::deal::hand::StackHandStd;
/// use brydz_core::karty::cards::{ACE_CLUBS, JACK_SPADES, QUEEN_DIAMONDS, KING_HEARTS};
/// use brydz_core::karty::cards::Card;
/// use crate::brydz_core::deal::hand::Hand;
/// let mut hand = StackHandStd::new_empty();
/// hand.add_card(ACE_CLUBS);
/// hand.add_card( KING_HEARTS);
/// hand.add_card( QUEEN_DIAMONDS);
/// hand.add_card( JACK_SPADES);
/// let v: Vec<Card> = hand.into_iter().collect();
/// assert_eq!(v.len(), 4);
/// assert_eq!(v[0], ACE_CLUBS);
/// ```
impl Iterator for StackHandStdIterator{
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        /*
        let maxt_iterations = self.mask.leading_zeros() + 1;
        for _ in 0..=maxt_iterations{
            if self.mask & self.hand.cards != 0{
                let card = CardStd::from_mask(self.mask).unwrap();
                    self.mask <<=1;
                    return Some(card);
            }
            self.mask <= 1;
        }*/
        
        if !self.end{
            while self.mask != (LARGEST_MASK){
                if self.mask & self.hand.cards != 0{
                    let card = Card::from_mask(self.mask).unwrap();
                    self.mask <<=1;
                    return Some(card);
                }
                else{
                    self.mask<<=1;
                }
            }
            self.end = true;
            Card::from_mask(self.mask)

        }
        else{
            None
        }
        
        

    }
}

impl IntoIterator for StackHandStd{
    type Item = Card;

    type IntoIter = StackHandStdIterator;

    fn into_iter(self) -> Self::IntoIter {
        StackHandStdIterator::new(self)
    }
}

impl Hand for StackHandStd{
    type CardType = Card;

    fn add_card(&mut self, card: Self::CardType) -> Result<(), crate::error::HandError> {
        match self.contains(&card){
            true => Err(HandError::CardDuplicated),
            false => {
                self.cards |= card.mask();
                Ok(())
            }
        }
    }

    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), crate::error::HandError> {
        match self.contains(card){
            true => {
                self.cards ^= card.mask();
                Ok(())
            },
            false => Err(HandError::CardNotInHand)
        }
    }

    fn new_empty() -> Self {
        Self{cards: 0u64}
    }

    fn contains(&self, card: &Self::CardType) -> bool {
        card.mask() & self.cards != 0
    }

    fn len(&self) -> usize {
        self.cards.count_ones() as usize
    }
}

impl Display for StackHandStd{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v: Vec<Card> = self.into_iter().collect();
        write!(f,  "[")?;
        if f.alternate(){
            for e in v.into_iter(){
                write!(f, "{:#}, ", e)?;
            }


        }
        else{
            for e in v.into_iter(){
                write!(f, "{}, ", e)?;
            }
        }
        write!(f, "]")
    }
}