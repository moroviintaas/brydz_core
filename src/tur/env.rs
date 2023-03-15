use tur::comm::CommEndpoint;
use tur::env::{BroadcastingEnv, CommunicatingEnv, Environment, StatefulEnvironment};
use tur::state::env::EnvironmentState;
use crate::player::side::{Side, SideMap, SIDES};
use crate::tur::state::{ContractAction, ContractState, ContractStateUpdate};
use std::iter::IntoIterator;
use tur::state::State;
use crate::player::side::Side::*;

pub struct ContractEnv<S: EnvironmentState + ContractState, C: CommEndpoint>{
    state: S,
    comm: SideMap<C>
}

impl<S: EnvironmentState + ContractState, C: CommEndpoint> ContractEnv<S, C>{
    pub fn new(state: S, comm: SideMap<C>) -> Self{
        Self{state, comm}
    }
}

impl<S: EnvironmentState + ContractState, C: CommEndpoint> CommunicatingEnv for ContractEnv<S, C>{
    type Outward = C::OutwardType;
    type Inward = C::InwardType;
    type CommunicationError = C::Error;
    type AgentId = Side;

    fn send_to(&mut self, agent_id: &Self::AgentId, message: Self::Outward) -> Result<(), Self::CommunicationError> {
        self.comm[agent_id].send(message)
    }

    fn recv_from(&mut self, agent_id: &Self::AgentId) -> Result<Self::Inward, Self::CommunicationError> {
        self.comm[agent_id].recv()
    }

    fn try_recv_from(&mut self, agent_id: &Self::AgentId) -> Result<Self::Inward, Self::CommunicationError> {
        self.comm[agent_id].try_recv()
    }
}

impl<S: EnvironmentState + ContractState, C: CommEndpoint> BroadcastingEnv for ContractEnv<S, C>
where <C as CommEndpoint>::OutwardType: Clone{
    fn send_to_all(&mut self, message: Self::Outward) -> Result<(), Self::CommunicationError> {
        for s in SIDES{
            self.comm[&s].send(message.clone())?;
        }
        Ok(())
    }
}

impl<'a, S: EnvironmentState + ContractState, C: CommEndpoint> Environment<'a, Side> for ContractEnv<S, C>{
    type PlayerIterator = &'a [Side; 4];

    fn players(&self) -> Self::PlayerIterator {
        &SIDES
    }
}

impl<S: EnvironmentState<AgentId=Side> + ContractState + ContractState, C: CommEndpoint> StatefulEnvironment for ContractEnv<S, C>
where S: State<UpdateType = ContractStateUpdate>{
    type State = S;
    type Action = ContractAction;
    type UpdatesIterator = <[(Side, ContractStateUpdate);4] as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn process_action(&mut self, agent: &<Self::State as EnvironmentState>::AgentId, action: Self::Action) -> Result<Self::UpdatesIterator, <Self::State as State>::Error> {

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

