use serde::{Deserialize, Serialize};
use crate::cards::{Card};
use arrayvec::ArrayVec;
use rand::{thread_rng};
use crate::cards::figure::FIGURES;
use crate::cards::suit::SUITS;
use itertools::Itertools;
use std::ops::Index;
use rand::seq::SliceRandom;

pub const DECK_SIZE: usize = 52;
pub const QUARTER_SIZE: usize = 13usize;
pub const MAX_INDEX_IN_DEAL: usize = QUARTER_SIZE -1;

#[derive(Debug, Eq, PartialEq,  Clone, Serialize, Deserialize)]
pub struct Deck{
    cards: ArrayVec<Card, DECK_SIZE>
}
impl Deck{


    pub fn new_sorted_by_suits() -> Self{
        let v: ArrayVec<Card, DECK_SIZE> = ArrayVec::from_iter(SUITS.into_iter()
            .cartesian_product(FIGURES.into_iter())
            .map(|(s,f)| Card::new(f,s)));

        Self{cards: v}
    }
    pub fn new_sorted_by_figures() -> Self{
        let v: ArrayVec<Card, DECK_SIZE> = ArrayVec::from_iter(FIGURES.into_iter()
            .cartesian_product(SUITS.into_iter())
            .map(|(s,f)| Card::new(s,f)));

        Self{cards: v}
    }


    pub fn at(&self, index: usize) -> &Card{
        &self.cards[index]
    }

    pub fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn cards(&self) -> &ArrayVec<Card, DECK_SIZE>{
        &self.cards
    }

}

impl IntoIterator for Deck{
    type Item = Card;
    type IntoIter = arrayvec::IntoIter<Self::Item, DECK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl Index<usize> for Deck{
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}
/*
pub struct ReleasedDeck {
    cards: ArrayVec<ReleasedCard, DECK_SIZE>
}
impl ReleasedDeck{
    ///  Creates new Deck containing all unique 52 standard cards
    /// ```
    /// use bridge_core::cards::{figure::Figure, Card};
    /// use bridge_core::cards::suit::Suit;
    /// use bridge_core::play::deck::Deck;
    /// let deck = Deck::new_sorted();
    /// assert_eq!(deck.at(0), &Card::new(Figure::Ace, Suit::Spades));
    /// assert_ne!(deck.at(5), &Card::new(Figure::King, Suit::Spades));
    ///
    /// ```
    ///

    pub fn new_id_rand() -> Self{
        let mut rng = rand::thread_rng();
        let mut ids = HashSet::with_capacity(DECK_SIZE);
        while ids.len() < DECK_SIZE{
            ids.insert(rng.next_u64());
        }

        let v: ArrayVec<ReleasedCard, DECK_SIZE> = ArrayVec::from_iter(SUITS.into_iter()
            .cartesian_product(FIGURES.into_iter()).zip(ids.into_iter())
            .map(|((s,f),id)| ReleasedCard::new(Card::new(f,s), id)));



        Self{cards: v}
    }
}


 */

