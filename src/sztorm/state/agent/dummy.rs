use smallvec::{SmallVec, smallvec};
use karty::hand::{HandTrait, CardSet};
use crate::contract::{Contract, ContractMechanics, ContractParameters};
use crate::error::BridgeCoreError;
use crate::player::side::Side;
use crate::sztorm::state::{ContractAction, ContractStateUpdate, StateWithSide};
use log::debug;
use sztorm::agent::{InformationSet, ScoringInformationSet};
use sztorm::state::ConstructedState;
use crate::deal::DescriptionDeckDeal;
use crate::meta::HAND_SIZE;
use crate::sztorm::spec::ContractDP;

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
/*
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

 */

impl InformationSet<ContractDP> for ContractDummyState {
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

impl ScoringInformationSet<ContractDP> for ContractDummyState{
    type RewardType = i32;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.contract.total_tricks_taken_axis(self.side.axis()) as i32
    }

    fn penalty_for_illegal() -> Self::RewardType {
        -100
    }
}

impl StateWithSide for ContractDummyState{
    fn id(&self) -> Side {
        self.side
    }
}

impl ConstructedState<ContractDP, (Side,  ContractParameters, DescriptionDeckDeal,)> for ContractDummyState{

    fn construct_from(base: (Side, ContractParameters, DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;

        let contract = Contract::new(params);
        Self::new(side, descript.cards[&side] , contract)
    }
}
impl ConstructedState<ContractDP, (&Side,  &ContractParameters, &DescriptionDeckDeal)> for ContractDummyState{
    fn construct_from(base: (&Side, &ContractParameters, &DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;

        let contract = Contract::new(params.clone());
        Self::new(*side, descript.cards[side] , contract)
    }
}