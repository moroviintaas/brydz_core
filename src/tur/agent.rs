use std::io::Error;
use log::debug;
use tur::agent::{CommunicatingAgent, PolicyAgent, StatefulAgent};
use tur::automatons::policy::Policy;
use tur::comm::CommEndpoint;
use tur::error::CommError;
use tur::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
use tur::state::agent::AgentState;
use tur::state::State;
use crate::tur::state::ContractAction;

pub struct ContractAgent<S: AgentState, C: CommEndpoint, P: Policy>{
    state: S,
    comm: C,
    policy: P
}

impl<S: AgentState, C: CommEndpoint, P: Policy> ContractAgent<S, C, P>{
    pub fn new(state: S, comm: C, policy: P) -> Self{
        Self{state, comm, policy}
    }
}

impl<S: AgentState, C: CommEndpoint, P: Policy> StatefulAgent for ContractAgent<S, C, P>{
    type State = S;

    fn update(&mut self, state_update: <Self::State as State>::UpdateType) -> Result<(), <Self::State as State>::Error> {
        self.state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.state
    }
}

impl<S: AgentState, C: CommEndpoint, P: Policy<StateType = S>> PolicyAgent for ContractAgent<S, C, P>{
    type Act = <S as AgentState>::ActionType;

    fn select_action(&self) -> Option<Self::Act> {
        self.policy.select_action(&self.state)
    }
}

impl<S: AgentState, C: CommEndpoint, P: Policy<StateType = S>>
CommunicatingAgent for ContractAgent<S, C, P>
//Spec: ProtocolSpecification,
//where C: CommEndpoint<OutwardType=AgentMessage<Spec>, InwardType=EnvMessage<Spec>, Error=CommError>
{
    type Outward = C::OutwardType;
    type Inward = C::InwardType;
    type CommunicationError = C::Error;

    fn send(&mut self, message: Self::Outward) -> Result<(), Self::CommunicationError> {
        self.comm.send(message)
    }

    fn recv(&mut self) -> Result<Self::Inward, Self::CommunicationError> {
        self.comm.recv()
    }
}

