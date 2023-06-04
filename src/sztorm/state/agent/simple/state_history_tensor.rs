use std::cmp::Ordering;
use karty::cards::{ DECK_SIZE, STANDARD_DECK_CDHS};
use karty::hand::{HandTrait};
use karty::symbol::CardSymbol;
use crate::cards::trump::TrumpGen;
use crate::contract::ContractMechanics;
use crate::sztorm::state::{BuildStateHistoryTensor, ContractAgentInfoSetSimple};

impl BuildStateHistoryTensor for ContractAgentInfoSetSimple{
    fn contract_params(&self) -> [f32; DECK_SIZE + 1] {
        let mut result = [0.0; DECK_SIZE+1];
        result[0] = self.contract.contract_spec().bid().number() as f32;
        match self.contract.contract_spec().bid().trump(){
            TrumpGen::Colored(c) => {
                result[1] = 1.0;
                result[2] = c.position() as f32;
            }
            TrumpGen::NoTrump => {
                result[1] = 0.0;
                result[2] = 0.0;
            }
        };


        result
    }

    fn prediction(&self, _relative_side: u8) -> [f32; DECK_SIZE + 1] {
        let mut result = [0.25; DECK_SIZE+1];
        result[DECK_SIZE] = 0.0;
        result

    }

    fn actual_cards(&self) -> [f32; DECK_SIZE + 1] {
        let mut cards = [0.0;DECK_SIZE+1];
        /*for suit in SUITS{
            for figure in FIGURES{

            }
        }*/
        for c in STANDARD_DECK_CDHS{
            if self.hand.contains(&c){
                cards[c.position()] = 1.0;
            } else {
                cards[c.position()] = 0.0;
            }


        }
        cards[DECK_SIZE] = 1.0;
        cards
    }

    fn actual_dummy_cards(&self) -> [f32; DECK_SIZE + 1] {
        match self.dummy_hand{
            None => [0.0; DECK_SIZE+1],
            Some(dh) => {
                let mut result = [0.0; DECK_SIZE+1];
                for card in STANDARD_DECK_CDHS{
                    if dh.contains(&card){
                        result[card.position()] = 1.0;
                    }
                }
                result[DECK_SIZE] = 1.0;
                result
            }
        }
    }

    fn trick_cards(&self, trick_number: usize, relative_side: u8) -> [f32; DECK_SIZE + 1] {
        match self.contract.completed_tricks().len().cmp(&trick_number){
            Ordering::Less => {
                let trick = self.contract.completed_tricks()[trick_number];
                let card = trick[self.side. next_i(relative_side)].unwrap();
                let mut mask = [0.0; DECK_SIZE+1];
                mask[card.position()] = 1.0;
                mask[DECK_SIZE] = 1.0;
                mask
            }
            Ordering::Equal => {
                let mut mask = [0.0; DECK_SIZE+1];
                if let Some(c) = self.contract.current_trick()[self.side.next_i(relative_side)] {
                    mask[c.position()] = 1.0;
                    mask[DECK_SIZE] = 1.0;
                }
                mask
            }
            Ordering::Greater => {
                [0.0;DECK_SIZE+1]
            }
        }
    }
}