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

    pub fn init(cards: &mut Vec<CardStd>) -> Result<Self, BridgeError<FigureStd, SuitStd>>{
        if cards.len() < CardStd::CARD_SPACE{
            return Err(Distribution(DistributionError::TooFewCards(cards.len())))
        }
        Ok(Self{cards: cards.drain(0..CardStd::CARD_SPACE/4).collect()})

        
    }
}