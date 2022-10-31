

use karty::cards::Card2Sym;

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};


use super::BridgeCoreError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum FormatError{
    SerializeError,
    DeserializeError

}

impl<Card: Card2Sym>  From<FormatError> for BridgeCoreError<Card>{
    fn from(e: FormatError) -> Self {
        Self::Format(e)
    }
}