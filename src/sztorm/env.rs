use crate::player::side::{Side, SideMap, SIDES};
use crate::sztorm::state::{
    ContractAction,
    ContractState,
    ContractStateUpdate};
use std::iter::IntoIterator;
use log::warn;
use sztorm::{comm::CommEndpoint, Reward};
use sztorm::env::{
    BroadcastingEnv,
    CommunicatingEnv,
    EnvironmentState,
    EnvironmentStateUniScore,
    EnvironmentWithAgents,
    ScoreEnvironment,
    StatefulEnvironment};
use sztorm::protocol::{
    AgentMessage,
    DomainParameters,
    EnvMessage};
use sztorm::state::State;
use crate::error::BridgeCoreError;
use crate::player::side::Side::*;
use crate::sztorm::spec::ContractProtocolSpec;

pub struct ContractEnv<S: EnvironmentState<ContractProtocolSpec> + ContractState, C: CommEndpoint>{
    state: S,
    comm: SideMap<C>,
    penalties: SideMap<<ContractProtocolSpec as DomainParameters>::UniversalReward>
}

impl<
    S: EnvironmentState<ContractProtocolSpec> + ContractState,
    C: CommEndpoint>
ContractEnv<S, C>{
    pub fn new(state: S, comm: SideMap<C>) -> Self{
        Self{
            state,
            comm,
            penalties: SideMap::new_symmetric(
                <ContractProtocolSpec as DomainParameters>::UniversalReward::neutral())
        }
    }
    pub fn replace_state(&mut self, state: S){
        self.state = state;
    }
}

impl<
    S: EnvironmentState<ContractProtocolSpec> + ContractState,
    C: CommEndpoint<
        OutwardType=EnvMessage<ContractProtocolSpec>,
        InwardType=AgentMessage<ContractProtocolSpec>>>
CommunicatingEnv<ContractProtocolSpec> for ContractEnv< S, C>{

    type CommunicationError = C::Error;
    //type AgentId = Side;

    fn send_to(
        &mut self,
        agent_id: &Side,
        message: EnvMessage<ContractProtocolSpec>)
        -> Result<(), Self::CommunicationError> {

        self.comm[agent_id].send(message)
    }

    fn recv_from(&mut self, agent_id: &Side) -> Result<AgentMessage<ContractProtocolSpec>, Self::CommunicationError> {
        self.comm[agent_id].recv()
    }

    fn try_recv_from(&mut self, agent_id: &Side) -> Result<AgentMessage<ContractProtocolSpec>, Self::CommunicationError> {
        self.comm[agent_id].try_recv()
    }
}

impl<S: EnvironmentState<ContractProtocolSpec> + ContractState,
    C: CommEndpoint<
        OutwardType=EnvMessage<ContractProtocolSpec>,
        InwardType=AgentMessage<ContractProtocolSpec>>>
BroadcastingEnv<ContractProtocolSpec> for ContractEnv<S, C>
where <C as CommEndpoint>::OutwardType: Clone{

    fn send_to_all(&mut self, message: EnvMessage<ContractProtocolSpec>) -> Result<(), Self::CommunicationError> {
        for s in SIDES{
            match self.comm[&s].send(message.clone()){
                Ok(_) => {},
                Err(_e) => warn!("Failed sending to {s:}")
            }
        }
        Ok(())
    }
}

impl<
    S: EnvironmentState<ContractProtocolSpec> + ContractState,
    C: CommEndpoint>
EnvironmentWithAgents<ContractProtocolSpec> for ContractEnv<S, C>{

    type PlayerIterator = [Side; 4];

    fn players(&self) -> Self::PlayerIterator {
        SIDES
    }
}

impl<
    S: EnvironmentState<ContractProtocolSpec> + ContractState + ContractState,
    C: CommEndpoint>
StatefulEnvironment<ContractProtocolSpec> for ContractEnv<S, C>
where S: State<ContractProtocolSpec> {
    type State = S;
    type UpdatesIterator = <[(Side, ContractStateUpdate);4] as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn process_action(&mut self, agent: &Side, action: &ContractAction) -> Result<Self::UpdatesIterator, BridgeCoreError> {

        let state_update =
        if self.state.is_turn_of_dummy() && Some(*agent) == self.state.current_player(){
            ContractStateUpdate::new(self.state.dummy_side(), action.clone())
        } else {
            ContractStateUpdate::new(agent.to_owned(), action.clone())
        };
        self.state.update(state_update)?;
        Ok([(North,state_update),(East,state_update),(South,state_update), (West, state_update)].into_iter())
    }




}
impl<
    S: EnvironmentState<ContractProtocolSpec>
        + ContractState + EnvironmentStateUniScore<ContractProtocolSpec> ,
    C: CommEndpoint>
ScoreEnvironment<ContractProtocolSpec> for ContractEnv<S, C>
where S: State<ContractProtocolSpec> {
    fn process_action_penalise_illegal(
        &mut self,
        agent: &<ContractProtocolSpec as DomainParameters>::AgentId,
        action: <ContractProtocolSpec as DomainParameters>::ActionType,
        penalty_reward: <ContractProtocolSpec as DomainParameters>::UniversalReward)
        -> Result<Self::UpdatesIterator, <ContractProtocolSpec as DomainParameters>::GameErrorType> {
        let state_update =
        if self.state.is_turn_of_dummy() && Some(*agent) == self.state.current_player(){
            ContractStateUpdate::new(self.state.dummy_side(), action)
        } else {
            ContractStateUpdate::new(agent.to_owned(), action)
        };
        match self.state.update(state_update){
            Ok(_) => Ok([(North,state_update),(East,state_update),(South,state_update), (West, state_update)].into_iter()),
            Err(err) => {
                //self.state.add_player_penalty_reward(agent, &penalty_reward);
                self.penalties[agent] += &penalty_reward;
                Err(err)
            }
        }
    }

    fn actual_state_score_of_player(&self, agent: &<ContractProtocolSpec as DomainParameters>::AgentId) -> <ContractProtocolSpec as DomainParameters>::UniversalReward {
        self.state.state_score_of_player(agent)
    }

    fn actual_penalty_score_of_player(&self, agent: &<ContractProtocolSpec as DomainParameters>::AgentId) -> <ContractProtocolSpec as DomainParameters>::UniversalReward {
        self.penalties[agent]
    }

    fn actual_score_of_player(&self, agent: &Side) -> <ContractProtocolSpec as DomainParameters>::UniversalReward {
        self.state.state_score_of_player(agent)
    }

}


pub struct ContractProcessor{

}

