
use crate::auction::bid::Bid;

use crate::player::side::Side;



#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
pub enum Doubling{
    None,
    Double,
    ReDouble
}

#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
pub enum Call {
    Bid(Bid),
    Double,
    ReDouble,
    Pass
}
#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
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








