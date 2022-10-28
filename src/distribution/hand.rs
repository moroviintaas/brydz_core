use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use karty::cards::{Card2S, CardStd};
use karty::figures::{FigureStd};
use karty::suits::{SuitStd};
use crate::error::{BridgeCoreError, DistributionError};
use crate::error::BridgeCoreError::Distribution;

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct BridgeHand{
    //cant be generic for now, because generic types cannot take part in const expressions
    //cards: Vec<CardStd>
    cards: HashSet<CardStd>
}
impl BridgeHand{


    /// # Example:
    /// ```
    /// use brydz_core::distribution::hand::BridgeHand;
    /// use karty::cards::STANDARD_DECK;
    /// use karty::cards;
    ///
    /// let mut card_supply = Vec::from(STANDARD_DECK);
    /// let hand_north = BridgeHand::drain_full_from_vec(&mut card_supply).unwrap();
    /// let hand_east = BridgeHand::drain_full_from_vec(&mut card_supply).unwrap();
    /// let hand_south = BridgeHand::drain_full_from_vec(&mut card_supply).unwrap();
    /// let hand_west = BridgeHand::drain_full_from_vec(&mut card_supply).unwrap();
    /// assert!(hand_north.cards().contains(&cards::TWO_CLUBS));
    /// assert!(hand_east.cards().contains(&cards::FIVE_DIAMONDS));
    /// assert!(hand_south.cards().contains(&cards::EIGHT_HEARTS));
    /// assert!(hand_west.cards().contains(&cards::JACK_SPADES));
    /// ```
    pub fn drain_full_from_vec(cards: &mut Vec<CardStd>) -> Result<Self, BridgeCoreError<FigureStd, SuitStd>>{
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
    /// use brydz_core::distribution::hand::BridgeHand;
    /// use brydz_core::karty::cards::*;
    /// use brydz_core::karty::suits::SuitStd::Spades;
    /// let mut card_supply = Vec::from([ACE_SPADES, KING_HEARTS, QUEEN_DIAMONDS, JACK_CLUBS,
    ///     TEN_SPADES, NINE_HEARTS, EIGHT_DIAMONDS, SEVEN_CLUBS, SIX_SPADES, FIVE_HEARTS,
    ///     FOUR_DIAMONDS, THREE_CLUBS, TWO_SPADES]);
    /// let hand = BridgeHand::drain_full_from_vec(&mut card_supply).unwrap();
    /// let spades_in_hand = hand.cards_in_suit(&Spades);
    /// assert!(spades_in_hand.contains(&ACE_SPADES));
    /// assert!(!spades_in_hand.contains(&KING_HEARTS));
    /// ```
    pub fn cards_in_suit(&self, suit: &SuitStd) -> HashSet<CardStd>{
        self.cards.iter().filter(|x| x.suit() == suit).map(|x| x.to_owned()).collect()
    }
}

impl Display for BridgeHand{
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
/*
impl Index<usize> for BridgeHand{
    type Output = CardStd;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}*/
