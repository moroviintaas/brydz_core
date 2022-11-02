use std::fmt::Debug;
use karty::cards::{ Card2Sym};
use crate::contract::trick::{Trick};
use crate::player::side::Side;
use crate::player::axis::Axis;
use crate::contract::spec::ContractSpec;
use crate::error::DealError;


pub trait ContractMaintainer<Card: Card2Sym>: Clone + Debug{
    fn current_trick(&self) -> &Trick<Card>;
    fn contract_spec(&self) -> &ContractSpec<Card::Suit>;
    fn count_completed_tricks(&self) -> usize;
    fn insert_card(&mut self, side: Side, card: Card) -> Result<Side, DealError<Card>>;
    fn is_completed(&self) -> bool;
    fn completed_tricks(&self) -> Vec<Trick<Card>>;
    fn total_tricks_taken_side(&self, side: Side) -> usize;
    fn total_tricks_taken_axis(&self, axis: Axis) -> usize;
    fn current_side(&self) -> Option<Side>{
        self.current_trick().current_side()
    }
    fn declarer(&self) -> Side{
        self.contract_spec().declarer()
    }
    fn dummy(&self) -> Side{
        self.contract_spec().declarer().partner()
    }
}

