use tur::comm::SyncComm;
use tur::error::CommError;
use tur::protocol::{AgentMessage, EnvMessage};
use crate::tur::spec::ContractProtocolSpec;

pub type ContractAgentSyncComm = SyncComm<AgentMessage<ContractProtocolSpec>, EnvMessage<ContractProtocolSpec>, CommError>;
pub type ContractEnvSyncComm = SyncComm<EnvMessage<ContractProtocolSpec>, AgentMessage<ContractProtocolSpec>, CommError>;