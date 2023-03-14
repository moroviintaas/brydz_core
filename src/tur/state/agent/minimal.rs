use smallvec::SmallVec;
use karty::hand::{HandSuitedTrait, HandTrait, CardSet};
use crate::contract::{Contract, ContractMechanics};
use tur::state;
use crate::error::BridgeCoreError;
use crate::meta::HAND_SIZE;
use crate::player::side::Side;
use crate::tur::state::{ContractAction, ContractStateUpdate};
use log::debug;
#[derive(Debug, Clone)]
pub struct ContractAgentStateMin {
    side: Side,
    hand: CardSet,
    dummy_hand: Option<CardSet>,
    contract: Contract
}

impl ContractAgentStateMin{
    pub fn new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>) -> Self{
        Self{side, hand, dummy_hand, contract}
    }
}


impl state::State for ContractAgentStateMin {
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

impl state::agent::AgentState for ContractAgentStateMin {
    type ActionType = ContractAction;
    type ActionIteratorType = SmallVec<[ContractAction; HAND_SIZE]>;
    type Id = Side;

    fn available_actions(&self) -> Self::ActionIteratorType {
        match self.contract.current_side(){
            dec if dec == self.side => {

                match self.contract.current_trick().called_suit(){
                    None => self.hand.into_iter()
                         .map(|card| ContractAction::PlaceCard(card)).collect(),
                    Some(called) => match self.hand.contains_in_suit(&called){
                        true => self.hand.suit_iterator(&called)
                            .map(|card| ContractAction::PlaceCard(card)).collect(),
                        false => self.hand.into_iter()
                            .map(|card| ContractAction::PlaceCard(card)).collect()
                    }
                }
            },
            dummy if dummy == self.side.partner()  && dummy == self.contract.dummy()=> {

                if let Some(dh) = self.dummy_hand{
                    match self.contract.current_trick().called_suit(){
                            None => dh.into_iter()
                                 .map(|card| ContractAction::PlaceCard(card)).collect(),
                            Some(called) => match dh.contains_in_suit(&called){
                                true => dh.suit_iterator(&called)
                                     .map(|card| ContractAction::PlaceCard(card)).collect(),
                                false => dh.into_iter()
                                     .map(|card| ContractAction::PlaceCard(card)).collect()
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
}
