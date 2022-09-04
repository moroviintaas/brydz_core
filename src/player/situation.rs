use std::collections::HashSet;
use karty::cards::CardStd;
use karty::register::{RegisterCardStd};
use karty::suits::SuitStd;
use crate::deal::{Contract, DealMaintainer, RegDeal, RegDealStd, TrickStd};
use crate::distribution::hand::BridgeHand;
use crate::error::{DealError, DealErrorStd, Mismatch, TrickError};
use crate::error::TrickError::TrickFull;
use crate::player::side::Side;

pub struct Situation {
    side: Side,
    hand: BridgeHand,
    dummy_hand: BridgeHand,
    deal: RegDealStd

}

impl Situation {
    pub fn new(side: Side, hand: BridgeHand, contract: Contract<SuitStd>) -> Self{
        Self{side, hand, dummy_hand: BridgeHand::empty(), deal: RegDeal::new(contract)}
    }
    pub fn set_dummy(&mut self, dummy_hand: BridgeHand){
        self.dummy_hand = dummy_hand
    }
    pub fn mark_card_used(&mut self, side: Side, card: CardStd) -> Result<(), DealErrorStd>{
        match self.current_side(){
            Some(s) if s == side => {
                self.cards_dummy_mut().remove(&card);
                self.cards_hand_mut().remove(&card);
                self.deal.insert_card(side, card).map(|_ | ())
            },
            Some(s) => Err(DealError::TrickError(
                TrickError::ViolatedOrder(Mismatch{expected: s, found: side}))),
            None => Err(DealError::TrickError(TrickFull)),

        }

    }
    pub fn cards_hand(&self) -> &HashSet<CardStd>{
        self.hand.cards()
    }
    pub fn cards_dummy(&self) -> &HashSet<CardStd>{
        self.dummy_hand.cards()
    }
    pub fn side(&self) -> Side{
        self.side
    }
    pub fn used_cards(&self) -> &RegisterCardStd{
        &self.deal.used_cards()
    }
    pub fn hand(&self) -> &BridgeHand{
        &self.hand
    }
    pub fn dummy_hand(&self) -> &BridgeHand{
        &self.dummy_hand
    }
    pub fn current_side(&self) -> Option<Side>{
        self.deal.current_side()
    }
    pub fn deal(&self) -> &RegDealStd{
        &self.deal
    }
    pub fn current_trick(&self) -> &TrickStd{
        self.deal().current_trick()
    }

    fn cards_hand_mut(&mut self) -> &mut HashSet<CardStd>{
        self.hand.cards_mut()
    }

    fn cards_dummy_mut(&mut self) -> &mut HashSet<CardStd>{
        self.dummy_hand.cards_mut()
    }

    pub fn deal_consume(self) -> RegDealStd{
        self.deal
    }
    /*
    fn use_card(&mut self, card: &CardStd) -> Result<(), HandError>{
        if self.cards_hand_mut().remove(card){
            self.mark_card_used(card.to_owned());
            Ok(())
        }
        Err(HandError::CardNotInHand)
    }*/


}