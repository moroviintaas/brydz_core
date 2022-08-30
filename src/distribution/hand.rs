use std::ops::Index;
use karty::cards::{Card2S, CardStd};
use karty::figures::{FigureStd};
use karty::suits::{SuitStd};
use crate::error::{BridgeError, DistributionError};
use crate::error::BridgeError::Distribution;

pub struct BridgeHand{
    //cant be generic for now, because generic types cannot take part in const expressions
    cards: Vec<CardStd>
}
impl BridgeHand{


    /// # Example:
    /// ```
    /// use bridge_core::distribution::hand::BridgeHand;
    /// use karty::cards::STANDARD_DECK;
    /// use karty::cards;
    ///
    /// let mut card_supply = Vec::from(STANDARD_DECK);
    /// let hand_north = BridgeHand::init(&mut card_supply).unwrap();
    /// let hand_east = BridgeHand::init(&mut card_supply).unwrap();
    /// let hand_south = BridgeHand::init(&mut card_supply).unwrap();
    /// let hand_west = BridgeHand::init(&mut card_supply).unwrap();
    /// assert_eq!(hand_north[0], cards::TWO_CLUBS);
    /// assert_eq!(hand_east[0], cards::FIVE_DIAMONDS);
    /// assert_eq!(hand_south[0], cards::EIGHT_HEARTS);
    /// assert_eq!(hand_west[0], cards::JACK_SPADES);
    /// ```
    pub fn init(cards: &mut Vec<CardStd>) -> Result<Self, BridgeError<FigureStd, SuitStd>>{
        if cards.len() < CardStd::CARD_SPACE/4{
            return Err(Distribution(DistributionError::TooFewCards(cards.len())))
        }
        Ok(Self{cards: cards.drain(0..CardStd::CARD_SPACE/4).collect()})

    }
}

impl Index<usize> for BridgeHand{
    type Output = CardStd;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}
