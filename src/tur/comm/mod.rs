use tur::comm::SyncComm;
use tur::error::CommError;
use tur::protocol::{AgentMessage, EnvMessage};
use tur::state::StateUpdate;
use crate::tur::spec::ContractProtocolSpec;
use crate::tur::state::{ContractAction, ContractStateUpdate};

pub type ContractAgentSyncComm = SyncComm<AgentMessage<ContractProtocolSpec>, EnvMessage<ContractProtocolSpec>, CommError>;
pub type ContractEnvSyncComm = SyncComm<EnvMessage<ContractProtocolSpec>, AgentMessage<ContractProtocolSpec>, CommError>;