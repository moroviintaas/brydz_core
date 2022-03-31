use serde::{Deserialize, Serialize};
use crate::auction::contract;
use crate::player::side::Side;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum Doubling{
    None,
    Double,
    ReDouble
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum Call {
    Bid(contract::Bid),
    Double,
    ReDouble,
    Pass
}
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct CallEntry{
    player_side: Side,
    call: Call
}

impl CallEntry {
    pub fn new(player_side: Side, call: Call) -> Self{
        Self{ player_side, call}
    }
    pub fn player_side(&self)-> Side{
        self.player_side
    }
    pub fn call(&self) -> Call {
        self.call
    }
}








