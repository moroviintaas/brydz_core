use smallvec::SmallVec;
use karty::cards::Card;
use karty::hand::CardSet;
use crate::meta::DECK_SIZE;
use crate::player::side::{Side, SideMap};
use num_rational::Rational32;


pub trait HandInfo{
    //fn side(&self) -> Side;
    fn own_cards(&self) -> CardSet;
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct HandInfoSimple {
    //side: Side,
    own_cards: CardSet,
}

impl HandInfoSimple{
    //pub fn new(side: Side, cards: CardSet) -> Self{
    pub fn new(cards: CardSet) -> Self{
        Self{own_cards: cards}
    }
}

impl HandInfo for HandInfoSimple{
    /*fn side(&self) -> Side {
        self.side
    }

     */

    fn own_cards(&self) -> CardSet {
        self.own_cards
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct HandInfoSuspect {
    //side: Side,
    //cards_probs: SideMap<[Rational32; 52]>
    //cards_probs: SideMap<Vec<f64>>
    cards_probs: Vec<SideMap<f64>>
}

