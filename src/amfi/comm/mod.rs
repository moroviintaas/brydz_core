use amfi_core::comm::StdEndpoint;
use amfi_core::error::CommunicationError;
use amfi_core::domain::{AgentMessage, EnvironmentMessage};
use crate::amfi::spec::ContractDP;

pub type ContractAgentSyncComm = StdEndpoint<AgentMessage<ContractDP>, EnvironmentMessage<ContractDP>, CommunicationError<ContractDP>>;
pub type ContractEnvSyncComm = StdEndpoint<EnvironmentMessage<ContractDP>, AgentMessage<ContractDP>, CommunicationError<ContractDP>>;