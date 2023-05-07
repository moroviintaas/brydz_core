use sztorm::{CommunicatingAgent, ActingAgent, StatefulAgent, PolicyAgent};
use sztorm::Policy;
use sztorm::CommEndpoint;
use sztorm::error::CommError;
use sztorm::InformationSet;
use sztorm::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
use crate::error::BridgeCoreError;
use crate::player::side::Side;
use crate::sztorm::spec::ContractProtocolSpec;
use crate::sztorm::state::{ContractAction, ContractStateUpdate};

pub struct ContractAgent<S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec>>{
    state: S,
    comm: C,
    policy: P
}

impl< S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec>> ContractAgent<S, C, P>{
    pub fn new(state: S, comm: C, policy: P) -> Self{
        Self{state, comm, policy}
    }
    pub fn replace_state(&mut self, state: S){
        self.state = state;
    }
}

impl<S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec>> StatefulAgent<ContractProtocolSpec> for ContractAgent< S, C, P>{
    type State = S;

    fn update(&mut self, state_update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
        self.state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.state
    }
}

impl< S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec, StateType = S>> ActingAgent<ContractProtocolSpec> for ContractAgent<S, C, P>{


    fn take_action(&self) -> Option<ContractAction> {
        self.policy.select_action(&self.state)
    }
}

impl<S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec, StateType = S>> PolicyAgent<ContractProtocolSpec> for ContractAgent<S, C, P>{
    type Policy = P;

    fn policy(&self) -> &Self::Policy {
        &self.policy
    }
}

impl<Spec: ProtocolSpecification,S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec, StateType = S>>
CommunicatingAgent<ContractProtocolSpec> for ContractAgent<S, C, P>
//Spec: ProtocolSpecification,
where C: CommEndpoint<OutwardType=AgentMessage<ContractProtocolSpec>, InwardType=EnvMessage<ContractProtocolSpec>, Error=CommError<Spec>>
{
    type CommunicationError = C::Error;

    fn send(&mut self, message: AgentMessage<ContractProtocolSpec>) -> Result<(), Self::CommunicationError> {
        self.comm.send(message)
    }

    fn recv(&mut self) -> Result<EnvMessage<ContractProtocolSpec>, Self::CommunicationError> {
        self.comm.recv()
    }
}

impl<S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec>> sztorm::DistinctAgent<ContractProtocolSpec> for ContractAgent<S, C, P>{
    //type Id = S::Id;

    fn id(&self) -> &Side {
        self.state().id()
    }
}

