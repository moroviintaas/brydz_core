use std::ops::Index;
use karty::hand::{CardSet, FuzzyCardSet};
use crate::player::side::{Side, SideMap};


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
pub struct CardDistribution {
    //side: Side,
    //cards_probs: SideMap<[Rational32; 52]>
    //#[serde(with = "BigArray")]
    //cards_probs: SideMap<[f64; DECK_SIZE]>
    side_probabilities: SideMap<FuzzyCardSet>
    //cards_probs: Vec<SideMap<f64>>
}

impl Index<Side> for CardDistribution {
    type Output = FuzzyCardSet;

    fn index(&self, index: Side) -> &Self::Output {
        &self.side_probabilities[&index]
    }
}
/*
impl Index<(Side, usize)> for CardDistribution {
    type Output = f32;

    fn index(&self, index: (Side, usize)) -> &Self::Output {
        &self.side_probabilities[&index.0].probabilities()[index.1]
    }
}*/

