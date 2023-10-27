use amfi::comm::SyncComm;
use amfi::error::CommunicationError;
use amfi::domain::{AgentMessage, EnvMessage};
use crate::amfi::spec::ContractDP;

pub type ContractAgentSyncComm = SyncComm<AgentMessage<ContractDP>, EnvMessage<ContractDP>, CommunicationError<ContractDP>>;
pub type ContractEnvSyncComm = SyncComm<EnvMessage<ContractDP>, AgentMessage<ContractDP>, CommunicationError<ContractDP>>;