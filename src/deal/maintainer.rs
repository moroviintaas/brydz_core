use karty::cards::Card;
use karty::figures::Figure;
use karty::suits::Suit;
use crate::deal::trick::{Trick};
use crate::player::side::Side;
use crate::player::axis::Axis;
use crate::deal::contract::Contract;
use crate::error::DealError;


pub trait DealMaintainer<F: Figure, S: Suit>{
    fn current_trick(&self) -> &Trick<F, S>;
    fn contract(&self) -> &Contract<S>;
    fn count_completed_tricks(&self) -> usize;
    fn insert_card(&mut self, side: Side, card: Card<F, S>) -> Result<Side, DealError<F, S>>;
    fn is_completed(&self) -> bool;
    fn completed_tricks(&self) -> Vec<Trick<F,S>>;
    fn total_tricks_taken_side(&self, side: Side) -> usize;
    fn total_tricks_taken_axis(&self, axis: Axis) -> usize;
    fn current_side(&self) -> Option<Side>{
        self.current_trick().current_side()
    }
    fn declarer(&self) -> Side{
        self.contract().declarer()
    }
    fn dummy(&self) -> Side{
        self.contract().declarer().partner()
    }
}
