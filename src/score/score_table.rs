use karty::figures::Figure;
use karty::suits::Suit;
use crate::deal::{DealMaintainer};
use crate::error::BridgeError;
use crate::player::axis::Axis;

pub trait Score<Co: DealMaintainer<F, S>, F:Figure, S: Suit>: Default{
    fn winner_axis(&self) -> Option<Axis>;
    fn update(&mut self, deal: &Co) -> Result<(), BridgeError<F, S>>;
    fn points(&self, axis: &Axis) -> i32;
}

pub trait ScoreIngredient<Co: DealMaintainer<F, S>, F:Figure, S: Suit>{
    fn calculate(&self, contract_overseer: Co, vulnerability: bool) -> Result<i32, BridgeError<F, S>>;
}