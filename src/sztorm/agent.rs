use sztorm::{CommunicatingAgent, ActingAgent, StatefulAgent, PolicyAgent, DomainEnvironment};
use sztorm::Policy;
use sztorm::CommEndpoint;
use sztorm::InformationSet;
use sztorm::State;
use crate::sztorm::spec::ContractProtocolSpec;

pub struct ContractAgent<S: InformationSet, C: CommEndpoint, P: Policy>{
    state: S,
    comm: C,
    policy: P
}

impl<S: InformationSet, C: CommEndpoint, P: Policy> ContractAgent<S, C, P>{
    pub fn new(state: S, comm: C, policy: P) -> Self{
        Self{state, comm, policy}
    }
}

impl<S: InformationSet, C: CommEndpoint, P: Policy> StatefulAgent for ContractAgent<S, C, P>{
    type State = S;

    fn update(&mut self, state_update: <Self::State as State>::UpdateType) -> Result<(), <Self::State as State>::Error> {
        self.state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.state
    }
}

impl<S: InformationSet, C: CommEndpoint, P: Policy<StateType = S>> ActingAgent for ContractAgent<S, C, P>{
    type Act = <S as InformationSet>::ActionType;

    fn take_action(&self) -> Option<Self::Act> {
        self.policy.select_action(&self.state)
    }
}

impl<S: InformationSet, C: CommEndpoint, P: Policy<StateType = S>> PolicyAgent for ContractAgent<S, C, P>{
    type Policy = P;

    fn policy(&self) -> &Self::Policy {
        &self.policy
    }
}

impl<S: InformationSet, C: CommEndpoint, P: Policy<StateType = S>>
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

impl<S: InformationSet, C: CommEndpoint, P: Policy> sztorm::DistinctAgent for ContractAgent<S, C, P>{
    type Id = S::Id;

    fn id(&self) -> &Self::Id {
        self.state().id()
    }
}

