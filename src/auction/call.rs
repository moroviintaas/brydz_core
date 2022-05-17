use carden::suits::Suit;
use crate::auction::bid::Bid;

use crate::player::side::Side;



#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
pub enum Doubling{
    None,
    Double,
    ReDouble
}

#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
pub enum Call<S: Suit> {
    Bid(Bid<S>),
    Double,
    ReDouble,
    Pass
}
#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
pub struct CallEntry<S: Suit>{
    player_side: Side,
    call: Call<S>
}

impl<S: Suit> CallEntry<S> {
    pub fn new(player_side: Side, call: Call<S>) -> Self{
        Self{ player_side, call}
    }
    pub fn player_side(&self)-> Side{
        self.player_side
    }
    pub fn call(&self) -> &Call<S> {
        &self.call
    }
}








