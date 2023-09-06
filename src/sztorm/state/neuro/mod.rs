mod as_state_history_tensor;
mod action_convert;

use tch::Tensor;
pub use as_state_history_tensor::*;
pub use action_convert::*;
use karty::cards::Card2SymTrait;
use karty::symbol::CardSymbol;
use sztorm_rl::tensor_repr::{ConvertToTensor, WayToTensor};
use crate::sztorm::state::ContractAction;

