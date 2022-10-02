use karty::figures::Figure;
use karty::suits::Suit;


#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};


use super::BridgeError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum FormatError{
    SerializeError,
    DeserializeError

}

impl<F:Figure, S:Suit>  From<FormatError> for BridgeError<F, S>{
    fn from(e: FormatError) -> Self {
        Self::Format(e)
    }
}