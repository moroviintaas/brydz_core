use sztorm::protocol::ProtocolSpecification;
use crate::error::BridgeCoreError;
use crate::player::side::Side;
use crate::sztorm::state::{ContractAction, ContractStateUpdate};

#[derive(Clone, Copy, Debug)]
pub struct ContractProtocolSpec{

}

impl ProtocolSpecification for ContractProtocolSpec{
    type ActionType = ContractAction;
    type GameErrorType = BridgeCoreError;
    type UpdateType = ContractStateUpdate;
    type AgentId = Side;
}