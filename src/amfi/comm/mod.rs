use amfi::comm::StdEndpoint;
use amfi::error::CommunicationError;
use amfi::domain::{AgentMessage, EnvironmentMessage};
use crate::amfi::spec::ContractDP;

pub type ContractAgentSyncComm = StdEndpoint<AgentMessage<ContractDP>, EnvironmentMessage<ContractDP>, CommunicationError<ContractDP>>;
pub type ContractEnvSyncComm = StdEndpoint<EnvironmentMessage<ContractDP>, AgentMessage<ContractDP>, CommunicationError<ContractDP>>;