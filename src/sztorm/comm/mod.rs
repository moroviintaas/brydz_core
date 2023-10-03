use sztorm::comm::SyncComm;
use sztorm::error::CommError;
use sztorm::domain::{AgentMessage, EnvMessage};
use crate::sztorm::spec::ContractDP;

pub type ContractAgentSyncComm = SyncComm<AgentMessage<ContractDP>, EnvMessage<ContractDP>, CommError<ContractDP>>;
pub type ContractEnvSyncComm = SyncComm<EnvMessage<ContractDP>, AgentMessage<ContractDP>, CommError<ContractDP>>;