use smallvec::SmallVec;
use karty::hand::{HandSuitedTrait, HandTrait, CardSet};
use crate::contract::{Contract, ContractMechanics};
use crate::error::BridgeCoreError;
use crate::meta::HAND_SIZE;
use crate::player::side::Side;
use crate::sztorm::state::{ContractAction, ContractStateUpdate};
use log::debug;
use karty::cards::Card2SymTrait;

#[derive(Debug, Clone)]
pub struct ContractAgentInfoSetSimple {
    side: Side,
    hand: CardSet,
    dummy_hand: Option<CardSet>,
    contract: Contract
}

impl ContractAgentInfoSetSimple {
    pub fn new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>) -> Self{
        Self{side, hand, dummy_hand, contract}
    }
}


impl sztorm::State for ContractAgentInfoSetSimple {
    type UpdateType = ContractStateUpdate;
    type Error = BridgeCoreError;

    fn update(&mut self, update: Self::UpdateType) -> Result<(), Self::Error> {
        //debug!("Agent {} received state update: {:?}", self.side, &update);
        let (side, action) = update.into_tuple();
        match action{
            ContractAction::ShowHand(dhand) => match side{
                s if s == self.contract.dummy() => match self.dummy_hand{
                    Some(_) => panic!("Behavior when dummy shows hand second time"),
                    None => {
                        self.dummy_hand = Some(dhand);
                        Ok(())
                    }

                }
                _ => panic!("Non defined behaviour when non dummy shows hand.")

            }
            ContractAction::PlaceCard(card) => {
                let actual_side = match self.contract.dummy() == self.contract.current_side(){
                    false => side,
                    true => match side == self.contract.dummy().partner(){
                        true => self.contract.dummy(),
                        false => side
                    }
                };
                debug!("Agent {:?}: actual_side: {:?}", &self.side, &actual_side);
                self.contract.insert_card(actual_side, card)?;
                if actual_side == self.side{
                    self.hand.remove_card(&card)?
                }
                if actual_side == self.contract.dummy(){
                    if let Some(ref mut dh) = self.dummy_hand{
                        dh.remove_card(&card)?
                    }
                }
                Ok(())

            }
        }
    }

    fn is_finished(&self) -> bool {
        self.contract.is_completed()
    }
}

impl sztorm::InformationSet for ContractAgentInfoSetSimple {
    type ActionType = ContractAction;
    type ActionIteratorType = SmallVec<[ContractAction; HAND_SIZE]>;
    type Id = Side;
    type RewardType = u32;

    fn available_actions(&self) -> Self::ActionIteratorType {
        match self.contract.current_side(){
            dec if dec == self.side => {

                match self.contract.current_trick().called_suit(){
                    None => self.hand.into_iter()
                         .map( ContractAction::PlaceCard).collect(),
                    Some(called) => match self.hand.contains_in_suit(&called){
                        true => self.hand.suit_iterator(&called)
                            .map(ContractAction::PlaceCard).collect(),
                        false => self.hand.into_iter()
                            .map(ContractAction::PlaceCard).collect()
                    }
                }
            },
            dummy if dummy == self.side.partner()  && dummy == self.contract.dummy()=> {

                if let Some(dh) = self.dummy_hand{
                    match self.contract.current_trick().called_suit(){
                            None => dh.into_iter()
                                 .map(ContractAction::PlaceCard).collect(),
                            Some(called) => match dh.contains_in_suit(&called){
                                true => dh.suit_iterator(&called)
                                     .map(ContractAction::PlaceCard).collect(),
                                false => dh.into_iter()
                                     .map( ContractAction::PlaceCard).collect()
                            }
                        }
                } else {
                    SmallVec::new()
                }

            },
            _ => SmallVec::new()
        }
    }

    fn id(&self) -> &Self::Id {
        &self.side
    }

    fn is_action_valid(&self, action: &Self::ActionType) -> bool {
        match action{
            ContractAction::ShowHand(_h) => {
                self.contract.dummy() == self.side
            }
            ContractAction::PlaceCard(c) => match self.hand.contains(c){
                true => match self.contract.current_trick().called_suit(){
                    None => true,
                    Some(s) => {
                        if s == c.suit(){
                            true
                        } else {
                            !self.hand.contains_in_suit(&s)
                        }
                    }
                }
                false => false
            }
        }
    }

    fn current_reward(&self) -> Self::RewardType {
        self.contract.total_tricks_taken_axis(self.side.axis())
    }
}
