use karty::figures::Figure;
use karty::suits::Suit;
use crate::deal::{Contract, DealMaintainer};
use crate::error::BridgeError;
use crate::player::axis::Axis;

pub trait ScoreTracker<Co: DealMaintainer<F, S>, F:Figure, S: Suit>: Default{
    fn winner_axis(&self) -> Option<Axis>;
    fn update(&mut self, deal: &Co) -> Result<(), BridgeError<F, S>>;
    fn points(&self, axis: &Axis) -> i32;
}

pub trait ScoreIngredient<S: Suit>{
    fn calculate(&self, contract: &Contract<S>, taken: u8, vulnerability: bool) -> i32;
}