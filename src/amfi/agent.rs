use amfi::agent::TracingAgentGen;
use crate::amfi::spec::ContractDP;

pub type TracingContractAgent<C, P> = TracingAgentGen<ContractDP, P, C>;
