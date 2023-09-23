mod simple;
mod dummy;
mod hand_info;
//#[cfg(feature = "fuzzy")]
mod fuzzy_card_set;
mod traits;
mod assuming;
mod tensor_convert;
mod state_id;
mod all_knowing;

pub use simple::*;
pub use dummy::*;
pub use hand_info::*;
pub use fuzzy_card_set::*;
pub use traits::*;
pub use tensor_convert::*;
pub use state_id::*;
pub use all_knowing::*;
pub use assuming::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError{
    #[error("Convert from tensor error")]
    ConvertFromTensor,
    #[error("Convert to tensor error")]
    ConvertToTensor
}




