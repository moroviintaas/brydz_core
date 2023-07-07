use sztorm::agent::{ActingAgent, CommunicatingAgent, DistinctAgent, GameTrace, GameTraceLine, Policy, PolicyAgent, RewardedAgent, StatefulAgent, TracingAgent};
use sztorm::{comm::CommEndpoint, Reward};
use sztorm::error::CommError;
use sztorm::protocol::{AgentMessage, EnvMessage, DomainParameters};
use sztorm::state::agent::InformationSet;
use crate::error::BridgeCoreError;
use crate::player::side::Side;
use crate::sztorm::spec::ContractProtocolSpec;
use crate::sztorm::state::{ContractAction, ContractStateUpdate};

pub struct ContractAgent<
    S: InformationSet<ContractProtocolSpec>,
    C: CommEndpoint,
    P: Policy<ContractProtocolSpec>>{

    state: S,
    comm: C,
    policy: P,
    //trace: SmallVec<[ContractTraceStep<S>;HAND_SIZE]>,
    trace: GameTrace<ContractProtocolSpec, S>,
    last_action: Option<<S::ActionIteratorType as IntoIterator>::Item>,
    //last_action_accumulated_reward: S::RewardType,
    last_action_state: Option<S>,
    constructed_universal_reward: <ContractProtocolSpec as DomainParameters>::UniversalReward,
    actual_universal_score: <ContractProtocolSpec as DomainParameters>::UniversalReward,
    //universal_rewards_stack: Vec<ContractProtocolSpec::UniversalReward>,

}

//#[allow(type_alias_bounds)]
//pub type ContractTraceStep<S: InformationSet<ContractProtocolSpec>> = (S, <S::ActionIteratorType as IntoIterator>::Item, S::RewardType );
impl< S: InformationSet<ContractProtocolSpec>, C: CommEndpoint, P: Policy<ContractProtocolSpec>> ContractAgent<S, C, P>{


    pub fn new(state: S, comm: C, policy: P) -> Self{
        Self{state, comm, policy,
            //trace: Default::default(),
            trace: GameTrace::new(),
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
    /*
    fn reset_trace(&mut self){
        //self.trace = Default::default();
        self.trace.clear();
        self.last_action = None;
        //self.last_action_accumulated_reward = Default::default();
    }

    pub fn game_trace(&self) -> &GameTrace<ContractProtocolSpec, S>{
        &self.trace
    }

     */

}

impl<S: InformationSet<ContractProtocolSpec>,
    C: CommEndpoint,
    P: Policy<ContractProtocolSpec>>
StatefulAgent<ContractProtocolSpec> for ContractAgent< S, C, P>{

    type State = S;

    fn update(&mut self, state_update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
        self.state.update(state_update)
    }

    fn state(&self) -> &Self::State {
        &self.state
    }
}

impl< S: InformationSet<ContractProtocolSpec>,
    C: CommEndpoint,
    P: Policy<ContractProtocolSpec, StateType = S>>
ActingAgent<ContractProtocolSpec> for ContractAgent<S, C, P>{



    fn take_action(&mut self) -> Option<ContractAction> {
        //debug!("Agent {} taking action", self.id());
        self.commit_trace();
        //self.last_action_accumulated_reward = self.state.current_subjective_score();
        let action = self.policy.select_action_mut(&self.state);
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

impl<S: InformationSet<ContractProtocolSpec>,
    C: CommEndpoint,
    P: Policy<ContractProtocolSpec, StateType = S>>
PolicyAgent<ContractProtocolSpec> for ContractAgent<S, C, P>{

    type Policy = P;

    fn policy(&self) -> &Self::Policy {
        &self.policy
    }

    fn policy_mut(&mut self) -> &mut Self::Policy {
        &mut self.policy
    }
}

impl<Spec: DomainParameters,
    S: InformationSet<ContractProtocolSpec>,
    C: CommEndpoint,
    P: Policy<ContractProtocolSpec, StateType = S>>
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

impl<
    S: InformationSet<ContractProtocolSpec>,
    C: CommEndpoint,
    P: Policy<ContractProtocolSpec>>
DistinctAgent<ContractProtocolSpec> for ContractAgent<S, C, P>{


    fn id(&self) -> &Side {
        self.state().id()
    }
}

impl<
    S: InformationSet<ContractProtocolSpec>,
    C: CommEndpoint,
    P: Policy<ContractProtocolSpec>>
RewardedAgent<ContractProtocolSpec> for ContractAgent<S, C, P>{
    fn current_universal_reward(&self) -> <ContractProtocolSpec as DomainParameters>::UniversalReward {
        self.constructed_universal_reward
    }

    fn current_universal_reward_add(&mut self, reward_fragment: &<ContractProtocolSpec as DomainParameters>::UniversalReward) {
        self.constructed_universal_reward += reward_fragment;
    }


    fn current_universal_score(&self) -> <ContractProtocolSpec as DomainParameters>::UniversalReward {
        self.actual_universal_score + self.constructed_universal_reward
    }

}


impl<S: InformationSet<ContractProtocolSpec>,
    C: CommEndpoint,
    P: Policy<ContractProtocolSpec>>
TracingAgent<ContractProtocolSpec, S> for ContractAgent<S, C, P> {
    fn reset_trace(&mut self) {
        self.trace.clear();
        self.last_action = None;
    }

    fn game_trajectory(&self) -> &GameTrace<ContractProtocolSpec, S> {
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
                GameTraceLine::new(
                    self.last_action_state.take().unwrap(),
                    prev_action,
                    self.state.current_subjective_score() - prev_subjective_score,
                    push_universal_reward));

        }
    }
}