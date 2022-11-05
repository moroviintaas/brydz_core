use karty::cards::Card2Sym;

use karty::suits::Suit;
use crate::contract::{ContractSpec, ContractMaintainer};
use crate::error::BridgeCoreError;
use crate::player::axis::Axis;

pub trait ScoreTracker<Co: ContractMaintainer<Card = Crd>, Crd: Card2Sym>: Default{
    fn winner_axis(&self) -> Option<Axis>;
    fn update(&mut self, deal: &Co) -> Result<(), BridgeCoreError<Crd>>;
    fn points(&self, axis: &Axis) -> i32;
}

pub trait ScoreIngredient<S: Suit>{
    fn calculate(&self, contract: &ContractSpec<S>, taken: u8, vulnerability: bool) -> i32;
}