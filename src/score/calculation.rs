use karty::figures::Figure;
use karty::suits::Suit;
use crate::contract::{ContractSpec, DealMaintainer};
use crate::error::BridgeCoreError;
use crate::player::axis::Axis;

pub trait ScoreTracker<Co: DealMaintainer<F, S>, F:Figure, S: Suit>: Default{
    fn winner_axis(&self) -> Option<Axis>;
    fn update(&mut self, deal: &Co) -> Result<(), BridgeCoreError<F, S>>;
    fn points(&self, axis: &Axis) -> i32;
}

pub trait ScoreIngredient<S: Suit>{
    fn calculate(&self, contract: &ContractSpec<S>, taken: u8, vulnerability: bool) -> i32;
}