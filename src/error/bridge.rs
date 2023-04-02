use std::fmt::{Display, Formatter};
use karty::cards::{Card2SymTrait, Card};
use karty::suits::{SuitTrait};

use crate::error::bidding::BiddingErrorGen;

#[cfg(feature="sztorm")]
use sztorm::error::{InternalGameError, SztormError};
#[cfg(feature="sztorm")]
use crate::sztorm::spec::ContractProtocolSpec;


use crate::error::contract::ContractErrorGen;
use crate::error::{DistributionError, CardErrorGen, ScoreError, TrickErrorGen};

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use crate::error::FormatError;



#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct Mismatch<T>{
    pub expected: T,
    pub found: T
}
impl<T: Copy> Copy for Mismatch<T>{}



impl<S: SuitTrait> Display for BiddingErrorGen<S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum BridgeCoreErrorGen<Card: Card2SymTrait>{
    Deal(ContractErrorGen<Card>),
    Bidding(BiddingErrorGen<Card::Suit>),
    Score(ScoreError),
    Trick(TrickErrorGen<Card>),
    Distribution(DistributionError),
    Hand(CardErrorGen<Card>),
    Format(FormatError),
    Custom(String),


}

impl<Card: Card2SymTrait> Display for BridgeCoreErrorGen<Card> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            BridgeCoreErrorGen::Deal(deal_error)=> match f.alternate(){
                true => write!(f, "BridgeError::DealError {{ {deal_error:#} }} " ),
                false => write!(f, "BridgeError::DealError {{ {deal_error} }} " ),
            }
            _ => {todo!()}
        }

    }
}

impl<Card: Card2SymTrait> std::error::Error for BridgeCoreErrorGen<Card>{}





pub type BridgeCoreError = BridgeCoreErrorGen<Card>;
/*
impl<F: Figure, S: Suit>  From<std::io::Error> for BridgeError<F, S>{
    fn from(e: std::io::Error) -> Self {
        Self::IO(e.kind())
    }
}
*/
/*#[cfg_attr(feature = "sztorm", derive(Writable, Readable))]
impl Into<TurError<ContractProtocolSpec>> for BridgeCoreError {
    fn into(self) -> TurError<ContractProtocolSpec> {
        TurError::GameError()
    }
}*/
#[cfg(feature = "sztorm")]
impl From<BridgeCoreError> for SztormError<ContractProtocolSpec>{
    fn from(value: BridgeCoreError) -> Self {
        Self::GameError(value)
    }
}

#[cfg(feature = "sztorm")]
impl  InternalGameError<ContractProtocolSpec> for BridgeCoreError{

}