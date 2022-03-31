use std::hash::{Hash};
use std::str::FromStr;
use crate::card::figure::{Figure};
use crate::card::suit::Suit;
use serde::{Deserialize, Serialize};
use crate::card::figure;
use crate::card::figure::Figure::{Ace, Jack, King, Numbered, Queen};
use crate::card::parser::parse_card;

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
/// use bridge_core::card::Card;
/// use bridge_core::card::figure::{Figure, NumberFigure};
/// use bridge_core::card::suit::Suit;
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

pub const TWO_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Numbered(figure::F2)};
pub const THREE_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Numbered(figure::F3)};
pub const FOUR_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Numbered(figure::F4)};
pub const FIVE_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Numbered(figure::F5)};
pub const SIX_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Numbered(figure::F6)};
pub const SEVEN_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Numbered(figure::F7)};
pub const EIGHT_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Numbered(figure::F8)};
pub const NINE_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Numbered(figure::F9)};
pub const TEN_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Numbered(figure::F10)};
pub const JACK_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Jack};
pub const QUEEN_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Queen};
pub const KING_CLUBS: Card = Card{ suit: Suit::Clubs, figure: King};
pub const ACE_CLUBS: Card = Card{ suit: Suit::Clubs, figure: Ace};

pub const TWO_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Numbered(figure::F2)};
pub const THREE_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Numbered(figure::F3)};
pub const FOUR_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Numbered(figure::F4)};
pub const FIVE_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Numbered(figure::F5)};
pub const SIX_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Numbered(figure::F6)};
pub const SEVEN_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Numbered(figure::F7)};
pub const EIGHT_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Numbered(figure::F8)};
pub const NINE_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Numbered(figure::F9)};
pub const TEN_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Numbered(figure::F10)};
pub const JACK_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Jack};
pub const QUEEN_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Queen};
pub const KING_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: King};
pub const ACE_DIAMONDS: Card = Card{ suit: Suit::Diamonds, figure: Ace};

pub const TWO_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Numbered(figure::F2)};
pub const THREE_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Numbered(figure::F3)};
pub const FOUR_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Numbered(figure::F4)};
pub const FIVE_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Numbered(figure::F5)};
pub const SIX_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Numbered(figure::F6)};
pub const SEVEN_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Numbered(figure::F7)};
pub const EIGHT_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Numbered(figure::F8)};
pub const NINE_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Numbered(figure::F9)};
pub const TEN_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Numbered(figure::F10)};
pub const JACK_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Jack};
pub const QUEEN_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Queen};
pub const KING_HEARTS: Card = Card{ suit: Suit::Hearts, figure: King};
pub const ACE_HEARTS: Card = Card{ suit: Suit::Hearts, figure: Ace};

pub const TWO_SPADES: Card = Card{ suit: Suit::Spades, figure: Numbered(figure::F2)};
pub const THREE_SPADES: Card = Card{ suit: Suit::Spades, figure: Numbered(figure::F3)};
pub const FOUR_SPADES: Card = Card{ suit: Suit::Spades, figure: Numbered(figure::F4)};
pub const FIVE_SPADES: Card = Card{ suit: Suit::Spades, figure: Numbered(figure::F5)};
pub const SIX_SPADES: Card = Card{ suit: Suit::Spades, figure: Numbered(figure::F6)};
pub const SEVEN_SPADES: Card = Card{ suit: Suit::Spades, figure: Numbered(figure::F7)};
pub const EIGHT_SPADES: Card = Card{ suit: Suit::Spades, figure: Numbered(figure::F8)};
pub const NINE_SPADES: Card = Card{ suit: Suit::Spades, figure: Numbered(figure::F9)};
pub const TEN_SPADES: Card = Card{ suit: Suit::Spades, figure: Numbered(figure::F10)};
pub const JACK_SPADES: Card = Card{ suit: Suit::Spades, figure: Jack};
pub const QUEEN_SPADES: Card = Card{ suit: Suit::Spades, figure: Queen};
pub const KING_SPADES: Card = Card{ suit: Suit::Spades, figure: King};
pub const ACE_SPADES: Card = Card{ suit: Suit::Spades, figure: Ace};
