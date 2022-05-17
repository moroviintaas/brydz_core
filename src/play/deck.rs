
use arrayvec::ArrayVec;
use rand::{thread_rng};
use itertools::Itertools;
use std::ops::Index;
use rand::seq::SliceRandom;
use carden::cards::Card;
use carden::figures::{FIGURES, FigureStd};
use carden::suits::{SUITS, SuitStd};


pub const DECK_SIZE: usize = 52;
pub const QUARTER_SIZE: usize = 13usize;
pub const MAX_INDEX_IN_DEAL: usize = QUARTER_SIZE -1;

/*pub trait DeckTrait<const S: usize>{

}*/


#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct Deck{
    cards: ArrayVec<Card<FigureStd, SuitStd>, DECK_SIZE>
    //cards: ArrayVec<ArrayVec<Card<F,S>, {F::NUMBER_OF_FIGURES}>, S::NUMBER_OF_SUITS>
}

impl Deck{



    pub fn new_sorted_by_suits() -> Self{
        let v: ArrayVec<Card<FigureStd, SuitStd>, DECK_SIZE> = ArrayVec::from_iter(SUITS.into_iter()
            .cartesian_product(FIGURES.into_iter())
            .map(|(s,f)| Card::new(f, s)));

        Self{cards: v}
    }
    pub fn new_sorted_by_figures() -> Self{
        let v: ArrayVec<Card<FigureStd, SuitStd>, DECK_SIZE> = ArrayVec::from_iter(FIGURES.into_iter()
            .cartesian_product(SUITS.into_iter())
            .map(|(s,f)| Card::new(s, f)));

        Self{cards: v}
    }


    pub fn at(&self, index: usize) -> &Card<FigureStd, SuitStd> {
        &self.cards[index]
    }

    pub fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn cards(&self) -> &ArrayVec<Card<FigureStd, SuitStd>, DECK_SIZE>{
        &self.cards
    }

}

impl IntoIterator for Deck{
    type Item = Card<FigureStd, SuitStd>;
    type IntoIter = arrayvec::IntoIter<Self::Item, DECK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl Index<usize> for Deck{
    type Output = Card<FigureStd, SuitStd>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

