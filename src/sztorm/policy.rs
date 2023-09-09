use sztorm::agent::Policy;
use crate::sztorm::spec::ContractDP;

pub trait ContractPolicy: Policy<ContractDP>{}

impl<P: Policy<ContractDP>> ContractPolicy for P{}