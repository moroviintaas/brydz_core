use smallvec::{SmallVec, smallvec};
use karty::hand::{HandTrait, CardSet};
use crate::contract::{Contract, ContractMechanics};
use crate::error::BridgeCoreError;
use crate::player::side::Side;
use crate::sztorm::state::{ContractAction, ContractStateUpdate, StateWithSide};
use log::debug;
use sztorm::state::agent::{InformationSet, ScoringInformationSet};
use sztorm::state::State;
use crate::meta::HAND_SIZE;
use crate::sztorm::spec::ContractProtocolSpec;

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

impl State<ContractProtocolSpec> for ContractDummyState {

    fn update(&mut self, update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
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


}

impl InformationSet<ContractProtocolSpec> for ContractDummyState {
    //type ActionType = ContractAction;
    type ActionIteratorType = SmallVec<[ContractAction; HAND_SIZE]>;
    //type Id = Side;
    //type RewardType = u32;

    fn available_actions(&self) -> Self::ActionIteratorType {
        match self.contract.current_side(){
            s if s == self.side => {
                smallvec![ContractAction::ShowHand(self.hand)]
            }
            _ => SmallVec::new()

        }
    }


    fn is_action_valid(&self, action: &ContractAction) -> bool {
        match action{
            ContractAction::ShowHand(_) => true,
            ContractAction::PlaceCard(_) => false
        }
    }

}

impl ScoringInformationSet<ContractProtocolSpec> for ContractDummyState{
    type RewardType = u32;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.contract.total_tricks_taken_axis(self.side.axis())
    }
}

impl StateWithSide for ContractDummyState{
    fn id(&self) -> Side {
        self.side
    }
}