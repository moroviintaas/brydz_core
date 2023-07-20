use sztorm::agent::AgentGenT;
use crate::sztorm::spec::ContractDP;

pub type TracingContractAgent<C, P> = AgentGenT<ContractDP, P, C>;
