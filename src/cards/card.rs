use std::hash::{Hash};
use std::str::FromStr;
use crate::cards::figure::{Figure, NumberFigure};
use crate::cards::suit::Suit;
use serde::{Deserialize, Serialize};
use crate::cards::figure::Figure::Numbered;
use crate::cards::parser::parse_card;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Hash)]
pub struct Card{
    suit: Suit,
    figure: Figure
}

impl Card{
    pub fn new(figure: Figure, suit: Suit) -> Self{
        Self{suit, figure}
    }

    pub fn suit(&self) -> Suit{
        self.suit
    }
    pub fn figure(&self) -> Figure{
        self.figure
    }
}

/// Parses Card from str
/// ```
/// use std::str::FromStr;
/// use bridge_core::cards::Card;
/// use bridge_core::cards::figure::{Figure, NumberFigure};
/// use bridge_core::cards::suit::Suit;
/// assert_eq!(Card::from_str("A s"), Ok(Card::new(Figure::Ace, Suit::Spades)));
/// assert_eq!(Card::from_str("4caa"), Ok(Card::new(Figure::Numbered(NumberFigure::new(4)), Suit::Clubs)));
/// assert!(Card::from_str("jq").is_err());
/// ```
impl FromStr for Card{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_card(s).map(|(_, card)| card).map_err(|e| format!("{}", e))
    }
}
/*
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct ReleasedCard{
    cards: Card,
    id: u64
}


impl ReleasedCard{
    pub fn new(cards: Card, id: u64) -> Self{
        ReleasedCard{cards, id}
    }
    pub fn suit(&self) -> Suit{
        self.cards.suit()
    }
    pub fn figure(&self) -> Figure{
        self.cards.figure()
    }
    pub fn cards(&self) -> Card{
        self.cards
    }

}
impl Hash for ReleasedCard{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
*/
