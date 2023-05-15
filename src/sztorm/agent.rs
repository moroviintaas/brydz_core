use smallvec::SmallVec;
use sztorm::{CommunicatingAgent, ActingAgent, StatefulAgent, PolicyAgent};
use sztorm::Policy;
use sztorm::CommEndpoint;
use sztorm::error::CommError;
use sztorm::InformationSet;
use sztorm::protocol::{AgentMessage, EnvMessage, ProtocolSpecification};
use crate::error::BridgeCoreError;
use crate::meta::HAND_SIZE;
use crate::player::side::Side;
use crate::sztorm::spec::ContractProtocolSpec;
use crate::sztorm::state::{ContractAction, ContractStateUpdate};

pub struct ContractAgent<S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec>>{
    state: S,
    comm: C,
    policy: P,
    trace: SmallVec<[(S, <S::ActionIteratorType as IntoIterator>::Item, S::RewardType );HAND_SIZE]>,
    last_action: Option<<S::ActionIteratorType as IntoIterator>::Item>,
    last_action_accumulated_reward: S::RewardType,
    last_action_state: Option<S>,

}

impl< S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec>> ContractAgent<S, C, P>{
    pub fn new(state: S, comm: C, policy: P) -> Self{
        Self{state, comm, policy,
            trace: Default::default(),
            last_action: None,
            last_action_accumulated_reward: Default::default(),
            last_action_state: None
        }
    }
    pub fn replace_state(&mut self, state: S) {
        self.state = state;
    }
    pub fn reset_trace(&mut self){
        self.trace = Default::default();
        self.last_action = None;
        self.last_action_accumulated_reward = Default::default();
    }

    pub fn trace(&self) -> &SmallVec<[(S, <S::ActionIteratorType as IntoIterator>::Item, S::RewardType );HAND_SIZE]>{
        &self.trace
    }

        /*
    pub fn policy_mut(&mut self) -> &mut P{
        &mut self.policy
    }

         */
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



    fn take_action(&mut self) -> Option<ContractAction> {
        //debug!("Agent {} taking action", self.id());
        if let Some(prev_action) = self.last_action.take(){
            self.trace.push((self.last_action_state.take().unwrap(), prev_action, self.state.current_score()- std::mem::take(&mut self.last_action_accumulated_reward)))
        }
        self.last_action_accumulated_reward = self.state.current_score();
        let action = self.policy.select_action_mut(&self.state);
        self.last_action = action;
        self.last_action_state = Some(self.state.clone());
        action.clone()
    }

    fn finalize(&mut self) {
        if let Some(prev_action) = self.last_action.take(){
            self.trace.push((self.last_action_state.take().unwrap(), prev_action, self.state.current_score()- std::mem::take(&mut self.last_action_accumulated_reward)))
        }
        self.last_action_accumulated_reward = self.state.current_score();
        self.last_action_state = Some(self.state.clone());
    }
}

impl<S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec, StateType = S>> PolicyAgent<ContractProtocolSpec> for ContractAgent<S, C, P>{
    type Policy = P;

    fn policy(&self) -> &Self::Policy {
        &self.policy
    }

    fn policy_mut(&mut self) -> &mut Self::Policy {
        &mut self.policy
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

