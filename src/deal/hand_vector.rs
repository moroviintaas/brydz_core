use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use karty::cards::{CardStd, Card2Sym};
use karty::suits::{SuitStd};
use crate::error::{BridgeCoreError, DistributionError, HandError};
use crate::error::BridgeCoreError::Distribution;

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use super::hand::Hand;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct HandVector{
    //cant be generic for now, because generic types cannot take part in const expressions
    //cards: Vec<CardStd>
    cards: HashSet<CardStd>
}

impl  IntoIterator for HandVector{
    type Item = CardStd;

    type IntoIter = std::collections::hash_set::IntoIter<CardStd>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}
impl HandVector{


    /// # Example:
    /// ```
    /// use brydz_core::deal::hand::HandVector;
    /// use karty::cards::STANDARD_DECK;
    /// use karty::cards;
    ///
    /// let mut card_supply = Vec::from(STANDARD_DECK);
    /// let hand_north = HandVector::drain_full_from_vec(&mut card_supply).unwrap();
    /// let hand_east = HandVector::drain_full_from_vec(&mut card_supply).unwrap();
    /// let hand_south = HandVector::drain_full_from_vec(&mut card_supply).unwrap();
    /// let hand_west = HandVector::drain_full_from_vec(&mut card_supply).unwrap();
    /// assert!(hand_north.cards().contains(&cards::TWO_CLUBS));
    /// assert!(hand_east.cards().contains(&cards::FIVE_DIAMONDS));
    /// assert!(hand_south.cards().contains(&cards::EIGHT_HEARTS));
    /// assert!(hand_west.cards().contains(&cards::JACK_SPADES));
    /// ```
    pub fn drain_full_from_vec(cards: &mut Vec<CardStd>) -> Result<Self, BridgeCoreError<CardStd>>{
        if cards.len() < CardStd::CARD_SPACE/4{
            return Err(Distribution(DistributionError::TooFewCards(cards.len())))
        }
        Ok(Self{cards: cards.drain(0..CardStd::CARD_SPACE/4).collect()})

    }
    pub fn cards(&self) -> &HashSet<CardStd>{
        &self.cards
    }
    pub(crate) fn cards_mut(&mut self) -> &mut HashSet<CardStd>{ &mut self.cards}
    pub fn empty() -> Self{
        Self{cards: HashSet::new()}
    }
    /// Returns subset of cards which are in specific `SuitStd`
    /// ```
    /// use brydz_core::deal::hand::HandVector;
    /// use brydz_core::karty::cards::*;
    /// use brydz_core::karty::suits::SuitStd::Spades;
    /// let mut card_supply = Vec::from([ACE_SPADES, KING_HEARTS, QUEEN_DIAMONDS, JACK_CLUBS,
    ///     TEN_SPADES, NINE_HEARTS, EIGHT_DIAMONDS, SEVEN_CLUBS, SIX_SPADES, FIVE_HEARTS,
    ///     FOUR_DIAMONDS, THREE_CLUBS, TWO_SPADES]);
    /// let hand = HandVector::drain_full_from_vec(&mut card_supply).unwrap();
    /// let spades_in_hand = hand.cards_in_suit(&Spades);
    /// assert!(spades_in_hand.contains(&ACE_SPADES));
    /// assert!(!spades_in_hand.contains(&KING_HEARTS));
    /// ```
    pub fn cards_in_suit(&self, suit: &SuitStd) -> HashSet<CardStd>{
        self.cards.iter().filter(|x| x.suit() == suit).map(|x| x.to_owned()).collect()
    }
}

impl Display for HandVector{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v: Vec<&CardStd> = self.cards().iter().collect();
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

impl Hand for HandVector{
    type CardType = CardStd;

    fn add_card(&mut self, card: Self::CardType) -> Result<(), crate::error::HandError> {
        match self.cards.contains(&card){
            true => Err(HandError::CardDuplicated),
            false => Ok(()),
        }
    }

    fn remove_card(&mut self, card: &Self::CardType) -> Result<(), crate::error::HandError> {
        match self.cards.remove(card){
            true => Ok(()),
            false => Err(HandError::CardNotInHand),
        }
    }

    fn new_empty() -> Self {
        Self{cards: HashSet::new()}
    }

    fn contains(&self, card: &Self::CardType) -> bool {
        self.cards.contains(&card)
    }

    fn len(&self) -> usize {
        self.cards.len()
    }
}
/*
impl Index<usize> for HandVectored{
    type Output = CardStd;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}*/

