use sztorm::comm::SyncComm;
use sztorm::error::CommError;
use sztorm::protocol::{AgentMessage, EnvMessage};
use crate::sztorm::spec::ContractProtocolSpec;

pub type ContractAgentSyncComm = SyncComm<AgentMessage<ContractProtocolSpec>, EnvMessage<ContractProtocolSpec>, CommError<ContractProtocolSpec>>;
pub type ContractEnvSyncComm = SyncComm<EnvMessage<ContractProtocolSpec>, AgentMessage<ContractProtocolSpec>, CommError<ContractProtocolSpec>>;