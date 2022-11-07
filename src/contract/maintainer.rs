use karty::cards::{Card2SymTrait};
use crate::contract::trick::{Trick};
use crate::player::side::Side;
use crate::player::axis::Axis;
use crate::contract::spec::ContractSpec;
use crate::error::ContractError;


pub trait ContractMaintainer{
    type Card: Card2SymTrait;
    
    fn current_trick(&self) -> &Trick<Self::Card>;
    fn contract_spec(&self) -> &ContractSpec<<Self::Card as Card2SymTrait>::Suit>;
    fn count_completed_tricks(&self) -> usize;
    fn insert_card(&mut self, side: Side, card: Self::Card) -> Result<Side, ContractError<Self::Card>>;
    fn is_completed(&self) -> bool;
    fn completed_tricks(&self) -> Vec<Trick<Self::Card>>;
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

