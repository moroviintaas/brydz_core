use std::ops::Index;
use karty::hand::CardSet;
use karty::cards::DECK_SIZE;
use crate::player::side::{Side, SideMap};
use serde_big_array::BigArray;


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
struct WrapCardProbs{
    #[serde(with = "BigArray")]
    pub probabilities: [f64; DECK_SIZE]
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct HandInfoSuspect {
    //side: Side,
    //cards_probs: SideMap<[Rational32; 52]>
    //#[serde(with = "BigArray")]
    //cards_probs: SideMap<[f64; DECK_SIZE]>
    side_probabilities: SideMap<WrapCardProbs>
    //cards_probs: Vec<SideMap<f64>>
}

impl Index<Side> for HandInfoSuspect{
    type Output = [f64; DECK_SIZE];

    fn index(&self, index: Side) -> &Self::Output {
        &self.side_probabilities[&index].probabilities
    }
}

impl Index<(Side, usize)> for HandInfoSuspect{
    type Output = f64;

    fn index(&self, index: (Side, usize)) -> &Self::Output {
        &self.side_probabilities[&index.0].probabilities[index.1]
    }
}

