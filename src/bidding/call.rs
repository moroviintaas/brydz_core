use std::fmt::{Display, Formatter};
use karty::suits::Suit;
use crate::bidding::bid::Bid;

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

impl<S: Suit+ Display> Display for Call<S>{
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!();
        /*
        if f.alternate(){
            match self{
                Call::Bid(bid) => write!(f, "Call::Bid{{ {:#} }}", bid),

            }

        }
        else{
            match self{
                Call::Bid(bid) => write!(f, "Call::Bid{{ {} }}", bid),
            }
        }*/
    }
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








