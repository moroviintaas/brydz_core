use serde::{Deserialize, Serialize};
use crate::card::{Card, ReleasedCard};
use std::vec::Vec;
use arrayvec::ArrayVec;
use rand::{RngCore, thread_rng};
use crate::card::figure::FIGURES;
use crate::card::suit::SUITS;
use itertools::Itertools;
use std::collections::{HashSet};
use std::ops::Index;
use rand::seq::SliceRandom;

pub const DECK_SIZE: usize = 52;

#[derive(Debug, Eq, PartialEq,  Clone, Serialize, Deserialize)]
pub struct Deck{
    cards: ArrayVec<ReleasedCard, DECK_SIZE>
}
impl Deck{
    ///  Creates new Deck containing all unique 52 standard cards
    /// ```
    /// use bridge_core::card::{figure::Figure, Card};
    /// use bridge_core::card::suit::Suit;
    /// use bridge_core::deck::Deck;
    /// let deck = Deck::new_id_rand();
    /// assert_eq!(deck.at(0).card(), Card::new(Figure::Ace, Suit::Spades));
    /// assert_ne!(deck.at(5).card(), Card::new(Figure::King, Suit::Spades));
    ///
    /// ```
    ///
    pub fn new_id_rand() -> Deck{
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

    pub fn at(&self, index: usize) -> &ReleasedCard{
        &self.cards[index]
    }

    pub fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn cards(&self) -> &ArrayVec<ReleasedCard, DECK_SIZE>{
        &self.cards
    }

}

impl IntoIterator for Deck{
    type Item = ReleasedCard;
    type IntoIter = arrayvec::IntoIter<Self::Item, DECK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl Index<usize> for Deck{
    type Output = ReleasedCard;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

