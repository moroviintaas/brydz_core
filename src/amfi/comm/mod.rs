use amfi::comm::SyncComm;
use amfi::error::CommError;
use amfi::domain::{AgentMessage, EnvMessage};
use crate::amfi::spec::ContractDP;

pub type ContractAgentSyncComm = SyncComm<AgentMessage<ContractDP>, EnvMessage<ContractDP>, CommError<ContractDP>>;
pub type ContractEnvSyncComm = SyncComm<EnvMessage<ContractDP>, AgentMessage<ContractDP>, CommError<ContractDP>>;