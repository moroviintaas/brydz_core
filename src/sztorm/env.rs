use crate::player::side::{Side, SideMap, SIDES};
use crate::sztorm::state::{
    ContractAction,
    ContractState};
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
use crate::error::BridgeCoreError;
use crate::sztorm::spec::ContractDP;

pub struct ContractEnv<S: EnvironmentState<ContractDP> + ContractState, C: CommEndpoint>{
    state: S,
    comm: SideMap<C>,
    penalties: SideMap<<ContractDP as DomainParameters>::UniversalReward>
}

impl<
    S: EnvironmentState<ContractDP> + ContractState,
    C: CommEndpoint>
ContractEnv<S, C>{
    pub fn new(state: S, comm: SideMap<C>) -> Self{
        Self{
            state,
            comm,
            penalties: SideMap::new_symmetric(
                <ContractDP as DomainParameters>::UniversalReward::neutral())
        }
    }
    pub fn replace_state(&mut self, state: S){
        self.state = state;
    }
}

impl<
    S: EnvironmentState<ContractDP> + ContractState,
    C: CommEndpoint<
        OutwardType=EnvMessage<ContractDP>,
        InwardType=AgentMessage<ContractDP>>>
CommunicatingEnv<ContractDP> for ContractEnv< S, C>{

    type CommunicationError = C::Error;
    //type AgentId = Side;

    fn send_to(
        &mut self,
        agent_id: &Side,
        message: EnvMessage<ContractDP>)
        -> Result<(), Self::CommunicationError> {

        self.comm[agent_id].send(message)
    }

    fn recv_from(&mut self, agent_id: &Side) -> Result<AgentMessage<ContractDP>, Self::CommunicationError> {
        self.comm[agent_id].recv()
    }

    fn try_recv_from(&mut self, agent_id: &Side) -> Result<AgentMessage<ContractDP>, Self::CommunicationError> {
        self.comm[agent_id].try_recv()
    }
}

impl<S: EnvironmentState<ContractDP> + ContractState,
    C: CommEndpoint<
        OutwardType=EnvMessage<ContractDP>,
        InwardType=AgentMessage<ContractDP>>>
BroadcastingEnv<ContractDP> for ContractEnv<S, C>
where <C as CommEndpoint>::OutwardType: Clone{

    fn send_to_all(&mut self, message: EnvMessage<ContractDP>) -> Result<(), Self::CommunicationError> {
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
    S: EnvironmentState<ContractDP> + ContractState,
    C: CommEndpoint>
EnvironmentWithAgents<ContractDP> for ContractEnv<S, C>{

    type PlayerIterator = [Side; 4];

    fn players(&self) -> Self::PlayerIterator {
        SIDES
    }
}

impl<
    S: EnvironmentState<ContractDP> + ContractState + ContractState,
    C: CommEndpoint>
StatefulEnvironment<ContractDP> for ContractEnv<S, C>
where S: EnvironmentState<ContractDP> {
    type State = S;
    //type Updates = <[(Side, ContractStateUpdate);4] as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn process_action(&mut self, agent: &Side, action: &ContractAction)
        -> Result<<Self::State as EnvironmentState<ContractDP>>::Updates, BridgeCoreError> {

        self.state.forward(*agent, *action)
    }
}


impl<
    S: EnvironmentState<ContractDP>
        + ContractState + EnvironmentStateUniScore<ContractDP> ,
    C: CommEndpoint>
ScoreEnvironment<ContractDP> for ContractEnv<S, C>
where S: EnvironmentState<ContractDP> {
    fn process_action_penalise_illegal(
        &mut self,
        agent: &<ContractDP as DomainParameters>::AgentId,
        action: &<ContractDP as DomainParameters>::ActionType,
        penalty_reward: <ContractDP as DomainParameters>::UniversalReward)

        -> Result<
            <<Self as StatefulEnvironment<ContractDP>>::State as EnvironmentState<ContractDP>>::Updates, <ContractDP as DomainParameters>::GameErrorType> {

        /*
        let state_update =
        if self.state.is_turn_of_dummy() && Some(*agent) == self.state.current_player(){
            ContractStateUpdate::new(self.state.dummy_side(), *action)
        } else {
            ContractStateUpdate::new(agent.to_owned(), *action)
        };




        match self.state.update(state_update){
            Ok(_) => Ok([(North,state_update),(East,state_update),(South,state_update), (West, state_update)].into_iter()),
            Err(err) => {
                //self.state.add_player_penalty_reward(agent, &penalty_reward);
                self.penalties[agent] += &penalty_reward;
                Err(err)
            }
        }
        */

        self.state.forward(*agent, *action).map_err(|e|{
            self.penalties[agent] += &penalty_reward;
            e
        })


    }

    fn actual_state_score_of_player(&self, agent: &<ContractDP as DomainParameters>::AgentId) -> <ContractDP as DomainParameters>::UniversalReward {
        self.state.state_score_of_player(agent)
    }

    fn actual_penalty_score_of_player(&self, agent: &<ContractDP as DomainParameters>::AgentId) -> <ContractDP as DomainParameters>::UniversalReward {
        self.penalties[agent]
    }

    fn actual_score_of_player(&self, agent: &Side) -> <ContractDP as DomainParameters>::UniversalReward {
        self.state.state_score_of_player(agent)
    }

}


pub struct ContractProcessor{

}

