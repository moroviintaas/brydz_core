use smallvec::{SmallVec, smallvec};
use karty::hand::{HandTrait, CardSet};
use crate::contract::{Contract, ContractMechanics};
use crate::error::BridgeCoreError;
use crate::player::side::Side;
use crate::sztorm::state::{ContractAction, ContractStateUpdate};
use log::debug;
use crate::meta::HAND_SIZE;

//#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ContractDummyState {
    side: Side,
    hand: CardSet,
    contract: Contract
}

impl ContractDummyState {
    pub fn new(side: Side, hand: CardSet, contract: Contract) -> Self{
        Self{side, hand, contract}
    }
}

impl sztorm::State for ContractDummyState {
    type UpdateType = ContractStateUpdate;
    type Error = BridgeCoreError;

    fn update(&mut self, update: Self::UpdateType) -> Result<(), Self::Error> {
        //debug!("Agent {} received state update: {:?}", self.side, &update);
        let (side, action) = update.into_tuple();

        match action{
            ContractAction::ShowHand(h) =>{
                debug!("Dummy ({}) got state update of shown hand {:#}", side, h);
                Ok(())

            }
            ContractAction::PlaceCard(card) => {
                self.contract.insert_card(side, card)?;
                if side == self.side{
                    self.hand.remove_card(&card)?
                }
                Ok(())
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.contract.is_completed()
    }
}

impl sztorm::InformationSet for ContractDummyState {
    type ActionType = ContractAction;
    type ActionIteratorType = SmallVec<[ContractAction; HAND_SIZE]>;
    type Id = Side;
    type RewardType = u32;

    fn available_actions(&self) -> Self::ActionIteratorType {
        match self.contract.current_side(){
            s if s == self.side => {
                smallvec![ContractAction::ShowHand(self.hand)]
            }
            _ => SmallVec::new()

        }
    }

    fn id(&self) -> &Self::Id {
        &self.side
    }

    fn is_action_valid(&self, action: &Self::ActionType) -> bool {
        match action{
            ContractAction::ShowHand(_) => true,
            ContractAction::PlaceCard(_) => false
        }
    }

    fn current_reward(&self) -> Self::RewardType {
        self.contract.total_tricks_taken_axis(self.side.axis())
    }
}