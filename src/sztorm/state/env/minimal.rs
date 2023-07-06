use karty::hand::{HandTrait, CardSet};
use crate::contract::{Contract, ContractMechanics};
use crate::error::BridgeCoreError;
use crate::sztorm::state::{ContractAction, ContractState, ContractStateUpdate};
use log::{debug};
use sztorm::{ActionProcessor, EnvironmentState, EnvironmentStateUniScore, State};
use sztorm::protocol::DomainParameters;
use crate::player::side::{Side};
use crate::player::side::Side::*;
use crate::sztorm::env::ContractProcessor;
use crate::sztorm::spec::ContractProtocolSpec;

#[derive(Clone)]
pub struct ContractEnvStateMin{
    dummy_hand: Option<CardSet>,
    contract: Contract,
}

impl ContractEnvStateMin{

    pub fn new(contract: Contract, dummy_hand: Option<CardSet>) -> Self{
        Self{dummy_hand, contract }
    }

    pub fn dummy_hand(&self) -> Option<&CardSet>{
        self.dummy_hand.as_ref()
    }

    pub fn contract(&self) -> &Contract{
        &self.contract
    }
    pub fn replace_contract(&mut self, contract: Contract){
        self.contract = contract
    }

}

impl ContractState for ContractEnvStateMin{
    fn dummy_side(&self) -> Side {
        self.contract.dummy()
    }

    fn current_side(&self) -> Side {
        self.contract.current_side()
    }
}

impl sztorm::State<ContractProtocolSpec> for ContractEnvStateMin{

    fn update(&mut self, update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
        debug!("Updating environment with {:?}", &update);
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
                self.contract.insert_card(actual_side, card)?;
                if side == self.contract.dummy(){
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

impl EnvironmentState<ContractProtocolSpec> for ContractEnvStateMin{

    fn current_player(&self) -> Option<Side> {
        match self.contract.is_completed(){
            true => None,
            false => Some(match self.contract.dummy() == self.contract.current_side(){
                true => match self.dummy_hand{
                    None => self.contract.dummy(),
                    Some(_) => self.contract.dummy().partner(),
                }
                false => self.contract.current_side()
            })
        }
    }

}

impl EnvironmentStateUniScore<ContractProtocolSpec> for ContractEnvStateMin{


    fn state_score_of_player(&self, agent: &Side) -> <ContractProtocolSpec as DomainParameters>::UniversalReward {
        self.contract.total_tricks_taken_axis(agent.axis()) as i32
    }
/*
    fn penalty_score_of_player(&self, agent: &<ContractProtocolSpec as DomainParameters>::AgentId) -> <ContractProtocolSpec as DomainParameters>::UniversalReward {
        self.penalties[agent]
    }

    fn score_of_player(&self, agent: &<ContractProtocolSpec as DomainParameters>::AgentId) -> <ContractProtocolSpec as DomainParameters>::UniversalReward {
        self.state_score_of_player(agent) + self.penalty_score_of_player(agent)
    }

    fn add_player_penalty_reward(&mut self, agent: &<ContractProtocolSpec as DomainParameters>::AgentId, penalty_reward: &<ContractProtocolSpec as DomainParameters>::UniversalReward) {
        self.penalties[agent] += penalty_reward
    }

 */
}

impl ActionProcessor<ContractProtocolSpec, ContractEnvStateMin > for ContractProcessor{
    fn process_action(&self, state: &mut ContractEnvStateMin, agent_id: &Side, action: ContractAction) -> Result<Vec<(Side, ContractStateUpdate)>, BridgeCoreError> {
        let state_update =
            if state.is_turn_of_dummy() && Some(*agent_id) == state.current_player(){
                ContractStateUpdate::new(state.dummy_side(), action)
            } else {
                ContractStateUpdate::new(*agent_id, action)
            };
        state.update(state_update)?;
        Ok(vec![(North,state_update),(East,state_update),(South,state_update), (West, state_update)])
    }


}
/*
impl ActionProcessorPenalising<ContractProtocolSpec, ContractEnvStateMin > for ContractProcessor{
    fn process_action_penalise_illegal(
        &self,
        state: &mut ContractEnvStateMin,
        agent_id: &Side,
        action: <ContractProtocolSpec as DomainParameters>::ActionType,
        penalty_reward: <ContractProtocolSpec as DomainParameters>::UniversalReward)
        -> Result<Vec<(Side, <ContractProtocolSpec as DomainParameters>::UpdateType)>, <ContractProtocolSpec as DomainParameters>::GameErrorType> {

        let state_update =
            if state.is_turn_of_dummy() && Some(*agent_id) == state.current_player(){
                ContractStateUpdate::new(state.dummy_side(), action)
            } else {
                ContractStateUpdate::new(*agent_id, action)
            };
        match state.update(state_update){
            Ok(_) => Ok(vec![(North,state_update),(East,state_update),(South,state_update), (West, state_update)]),
            Err(err) => {
                state.add_player_penalty_reward(agent_id, &penalty_reward);
                Err(err)
            }

        }

    }
}*/