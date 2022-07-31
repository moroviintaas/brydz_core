use karty::cards::Card;
use karty::figures::Figure;
use karty::register::Register;
use karty::suits::Suit;
use crate::contract::deal::Deal;
use crate::error::BridgeError;
use crate::player::axis::Axis;
use crate::player::side::Side;

pub trait Score<F: Figure, S:Suit, Um: Register<Card<F,S>>, Se: Register<(Side, S)>>{
    fn winner_axis(&self) -> Axis;
    fn update(&mut self, deal: &Deal<F, S, Um, Se>) -> Result<(), BridgeError<F, S>>;
}