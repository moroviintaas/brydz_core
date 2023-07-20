use sztorm::agent::{ActingAgent, Agent, CommunicatingAgent, AgentTrajectory, AgentTrace, Policy, PolicyAgent, RewardedAgent, StatefulAgent, TracingAgent};
use sztorm::{comm::CommEndpoint, Reward};
use sztorm::error::CommError;
use sztorm::protocol::{AgentMessage, EnvMessage, DomainParameters};
use sztorm::state::agent::{ScoringInformationSet};
use crate::error::BridgeCoreError;
use crate::player::side::Side;
use crate::sztorm::spec::ContractDP;
use crate::sztorm::state::{ContractAction, ContractStateUpdate, StateWithSide};

pub struct TracingContractAgent<
    S: ScoringInformationSet<ContractDP> + StateWithSide,
    C: CommEndpoint,
    P: Policy<ContractDP>>{

    state: S,
    comm: C,
    policy: P,
    //trace: SmallVec<[ContractTraceStep<S>;HAND_SIZE]>,
    trace: AgentTrajectory<ContractDP, S>,
    last_action: Option<<S::ActionIteratorType as IntoIterator>::Item>,
    //last_action_accumulated_reward: S::RewardType,
    last_action_state: Option<S>,
    //side: Side,
    constructed_universal_reward: <ContractDP as DomainParameters>::UniversalReward,
    actual_universal_score: <ContractDP as DomainParameters>::UniversalReward,
    //universal_rewards_stack: Vec<ContractProtocolSpec::UniversalReward>,

}

//#[allow(type_alias_bounds)]
//pub type ContractTraceStep<S: InformationSet<ContractProtocolSpec>> = (S, <S::ActionIteratorType as IntoIterator>::Item, S::RewardType );
impl<
    S: ScoringInformationSet<ContractDP> + StateWithSide,
    C: CommEndpoint,
    P: Policy<ContractDP>>
TracingContractAgent<S, C, P>{


    pub fn new(state: S, comm: C, policy: P) -> Self{
        Self{state, comm, policy,
            //trace: Default::default(),
            trace: AgentTrajectory::new(),
            last_action: None,
            //last_action_accumulated_reward: Default::default(),
            constructed_universal_reward: Reward::neutral(),
            last_action_state: None,
            actual_universal_score: Reward::neutral(),
        }
    }
    pub fn reset_state_and_trace(&mut self, state: S) {
        self.state = state;
        self.reset_trace();
    }


}

impl<S: ScoringInformationSet<ContractDP> + StateWithSide,
    C: CommEndpoint,
    P: Policy<ContractDP>>
StatefulAgent<ContractDP> for TracingContractAgent< S, C, P>{

    type State = S;

    fn update(&mut self, state_update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
        self.state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.state
    }
}

impl< S: ScoringInformationSet<ContractDP> + StateWithSide,
    C: CommEndpoint,
    P: Policy<ContractDP, StateType = S>>
ActingAgent<ContractDP> for TracingContractAgent<S, C, P>{



    fn take_action(&mut self) -> Option<ContractAction> {
        self.commit_trace();
        let action = self.policy.select_action(&self.state);
        self.last_action = action;
        self.last_action_state = Some(self.state.clone());
        action
    }

    fn finalize(&mut self) {
        self.commit_trace();
        //self.last_action_accumulated_reward = self.state.current_subjective_score();
        self.last_action_state = Some(self.state.clone());
    }
}

impl<S: ScoringInformationSet<ContractDP> + StateWithSide,
    C: CommEndpoint,
    P: Policy<ContractDP, StateType = S>>
PolicyAgent<ContractDP> for TracingContractAgent<S, C, P>{

    type Policy = P;

    fn policy(&self) -> &Self::Policy {
        &self.policy
    }

    fn policy_mut(&mut self) -> &mut Self::Policy {
        &mut self.policy
    }
}

impl<Spec: DomainParameters,
    S: ScoringInformationSet<ContractDP> + StateWithSide,
    C: CommEndpoint,
    P: Policy<ContractDP, StateType = S>>
CommunicatingAgent<ContractDP> for TracingContractAgent<S, C, P>
//Spec: ProtocolSpecification,
where C: CommEndpoint<OutwardType=AgentMessage<ContractDP>, InwardType=EnvMessage<ContractDP>, Error=CommError<Spec>>
{
    type CommunicationError = C::Error;

    fn send(&mut self, message: AgentMessage<ContractDP>) -> Result<(), Self::CommunicationError> {
        self.comm.send(message)
    }

    fn recv(&mut self) -> Result<EnvMessage<ContractDP>, Self::CommunicationError> {
        self.comm.recv()
    }
}

impl<
    S: ScoringInformationSet<ContractDP> + StateWithSide,
    C: CommEndpoint,
    P: Policy<ContractDP>>
Agent<ContractDP> for TracingContractAgent<S, C, P>{


    fn id(&self) -> Side {
        self.state().id()
    }
}

impl<
    S: ScoringInformationSet<ContractDP> + StateWithSide,
    C: CommEndpoint,
    P: Policy<ContractDP>>
RewardedAgent<ContractDP> for TracingContractAgent<S, C, P>{
    fn current_universal_reward(&self) -> <ContractDP as DomainParameters>::UniversalReward {
        self.constructed_universal_reward
    }

    fn current_universal_reward_add(&mut self, reward_fragment: &<ContractDP as DomainParameters>::UniversalReward) {
        self.constructed_universal_reward += reward_fragment;
    }


    fn current_universal_score(&self) -> <ContractDP as DomainParameters>::UniversalReward {
        self.actual_universal_score + self.constructed_universal_reward
    }

}


impl<S: ScoringInformationSet<ContractDP> + StateWithSide,
    C: CommEndpoint,
    P: Policy<ContractDP>>
TracingAgent<ContractDP, S> for TracingContractAgent<S, C, P> {
    fn reset_trace(&mut self) {
        self.trace.clear();
        self.last_action = None;
    }

    fn game_trajectory(&self) -> &AgentTrajectory<ContractDP, S> {
        &self.trace
    }

    fn commit_trace(&mut self) {
        if let Some(prev_action) = self.last_action.take(){
            //self.trace.push((self.last_action_state.take().unwrap(), prev_action, self.state.current_score()- std::mem::take(&mut self.last_action_accumulated_reward)))
            let prev_subjective_score = match &self.last_action_state{
                None => Reward::neutral(),
                Some(state) => state.current_subjective_score()
            };
            let push_universal_reward = std::mem::replace(&mut self.constructed_universal_reward, Reward::neutral());
            self.actual_universal_score  += push_universal_reward;
            self.trace.push_line(
                AgentTrace::new(
                    self.last_action_state.take().unwrap(),
                    prev_action,
                    self.state.current_subjective_score() - prev_subjective_score,
                    push_universal_reward));

        }
    }
}
