use amfi::agent::AgentGenT;
use crate::amfi::spec::ContractDP;

pub type TracingContractAgent<C, P> = AgentGenT<ContractDP, P, C>;
