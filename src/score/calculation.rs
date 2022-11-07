use karty::cards::Card2SymTrait;

use karty::suits::SuitTrait;
use crate::contract::{ContractSpec, ContractMaintainer};
use crate::error::BridgeCoreError;
use crate::player::axis::Axis;

pub trait ScoreTracker<Co: ContractMaintainer<Card = Crd>, Crd: Card2SymTrait>: Default{
    fn winner_axis(&self) -> Option<Axis>;
    fn update(&mut self, deal: &Co) -> Result<(), BridgeCoreError<Crd>>;
    fn points(&self, axis: &Axis) -> i32;
}

pub trait ScoreIngredient<S: SuitTrait>{
    fn calculate(&self, contract: &ContractSpec<S>, taken: u8, vulnerability: bool) -> i32;
}