use sztorm::{CommEndpoint, DomainEnvironment};
use sztorm::{BroadcastingEnv, CommunicatingEnv, EnvironmentWithAgents, StatefulEnvironment};
use sztorm::EnvironmentState;
use crate::player::side::{Side, SideMap, SIDES};
use crate::sztorm::state::{ContractAction,  ContractState, ContractStateUpdate};
use std::iter::IntoIterator;
use log::warn;
use sztorm::protocol::{AgentMessage, EnvMessage};
use sztorm::State;
use crate::error::BridgeCoreError;
use crate::player::side::Side::*;
use crate::sztorm::spec::ContractProtocolSpec;

pub struct ContractEnv<S: EnvironmentState<ContractProtocolSpec> + ContractState, C: CommEndpoint>{
    state: S,
    comm: SideMap<C>
}

impl<S: EnvironmentState<ContractProtocolSpec> + ContractState, C: CommEndpoint> ContractEnv<S, C>{
    pub fn new(state: S, comm: SideMap<C>) -> Self{
        Self{state, comm}
    }
    pub fn replace_state(&mut self, state: S){
        self.state = state;
    }
}

impl< S: EnvironmentState<ContractProtocolSpec> + ContractState,
    C: CommEndpoint<
        OutwardType=EnvMessage<ContractProtocolSpec>,
        InwardType=AgentMessage<ContractProtocolSpec>>>
CommunicatingEnv<ContractProtocolSpec> for ContractEnv< S, C>{

    type CommunicationError = C::Error;
    //type AgentId = Side;

    fn send_to(&mut self, agent_id: &Side, message: EnvMessage<ContractProtocolSpec>) -> Result<(), Self::CommunicationError> {
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

impl<S: EnvironmentState<ContractProtocolSpec> + ContractState, C: CommEndpoint> EnvironmentWithAgents<ContractProtocolSpec> for ContractEnv<S, C>{
    type PlayerIterator = [Side; 4];

    fn players(&self) -> Self::PlayerIterator {
        SIDES
    }
}

impl<S: EnvironmentState<ContractProtocolSpec> + ContractState + ContractState, C: CommEndpoint> StatefulEnvironment<ContractProtocolSpec> for ContractEnv<S, C>
where S: State<ContractProtocolSpec> {
    type State = S;
    type UpdatesIterator = <[(Side, ContractStateUpdate);4] as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn process_action(&mut self, agent: &Side, action: ContractAction) -> Result<Self::UpdatesIterator, BridgeCoreError> {

        let state_update =
        if self.state.is_turn_of_dummy() && Some(*agent) == self.state.current_player(){
            ContractStateUpdate::new(self.state.dummy_side(), action)
        } else {
            ContractStateUpdate::new(agent.to_owned(), action)
        };
        self.state.update(state_update)?;
        Ok([(North,state_update),(East,state_update),(South,state_update), (West, state_update)].into_iter())
    }
}
impl<S: EnvironmentState<ContractProtocolSpec> + ContractState, C: CommEndpoint> DomainEnvironment<ContractProtocolSpec> for ContractEnv<S, C>{
    //type DomainParameter<Spec> = ContractProtocolSpec;
}

pub struct ContractProcessor{

}



/*
impl<S: EnvironmentState<ContractProtocolSpec> + ContractState, C: CommEndpoint> ConstructedEnvironment<ContractProtocolSpec, C> for ContractEnv<S, C>{
    fn construct(state: S, mut env_comms: HashMap<Side, C>) -> Result<Self, SetupError<ContractProtocolSpec>> {

        let comm = SideMap::new(
            match env_comms.remove(&North){
                None => return Err(SetupError::MissingId(North)),
                Some(c) => c
            },
            match env_comms.remove(&East){
                None => return Err(SetupError::MissingId(East)),
                Some(c) => c
            },
            match env_comms.remove(&South){
                None => return Err(SetupError::MissingId(South)),
                Some(c) => c
            },
            match env_comms.remove(&West){
                None => return Err(SetupError::MissingId(West)),
                Some(c) => c
            },

        );
        Ok(Self{comm, state})



    }
}*/