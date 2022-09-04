use std::fmt::{Display, Formatter};
use karty::figures::Figure;
use karty::suits::Suit;
use crate::error::BridgeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreError{
    NegativeTrickNumber
}

impl Display for ScoreError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<F:Figure, S:Suit> From<ScoreError> for BridgeError<F, S>{
    fn from(e: ScoreError) -> Self {
        Self::Score(e)
    }
}