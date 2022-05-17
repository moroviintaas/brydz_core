
use std::str::FromStr;
use crate::card::figure::{Figure, FigureStd};
use crate::card::suit::{Suit, SuitStd};
use crate::card::figure;
use crate::card::figure::FigureStd::{Ace, Jack, King, Numbered, Queen};
use crate::card::parser::parse_card;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Card<F: Figure, S: Suit> {
    suit: S,
    figure: F
}

impl<F: Figure + Copy, S: Suit + Copy> Copy for Card<F, S>{}


impl<F:Figure, S: Suit > Card<F, S> {
    pub fn new(figure: F, suit: S) -> Self{
        Self{suit, figure}
    }

    pub fn suit(&self) -> &S {
        &self.suit
    }
    pub fn figure(&self) -> &F {
        &self.figure
    }



}

impl Card<FigureStd, SuitStd>{
    pub fn mask(&self) -> u64{

        self.figure.mask() << (self.suit.age() * 16)
    }
}

/// Parses Card from str
/// ```
/// use std::str::FromStr;
/// use bridge_core::card::Card;
/// use bridge_core::card::figure::{FigureStd, NumberFigureStd};
/// use bridge_core::card::suit::SuitStd;
/// assert_eq!(Card::from_str("A s"), Ok(Card::new(FigureStd::Ace, SuitStd::Spades)));
/// assert_eq!(Card::from_str("4caa"), Ok(Card::new(FigureStd::Numbered(NumberFigureStd::new(4)), SuitStd::Clubs)));
/// assert!(Card::from_str("jq").is_err());
/// ```
impl FromStr for Card<FigureStd, SuitStd> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_card(s).map(|(_, card)| card).map_err(|e| format!("{}", e))
    }
}

pub const TWO_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(figure::F2)};
pub const THREE_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(figure::F3)};
pub const FOUR_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(figure::F4)};
pub const FIVE_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(figure::F5)};
pub const SIX_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(figure::F6)};
pub const SEVEN_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(figure::F7)};
pub const EIGHT_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(figure::F8)};
pub const NINE_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(figure::F9)};
pub const TEN_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Numbered(figure::F10)};
pub const JACK_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Jack};
pub const QUEEN_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Queen};
pub const KING_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: King};
pub const ACE_CLUBS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Clubs, figure: Ace};

pub const TWO_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(figure::F2)};
pub const THREE_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(figure::F3)};
pub const FOUR_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(figure::F4)};
pub const FIVE_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(figure::F5)};
pub const SIX_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(figure::F6)};
pub const SEVEN_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(figure::F7)};
pub const EIGHT_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(figure::F8)};
pub const NINE_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(figure::F9)};
pub const TEN_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Numbered(figure::F10)};
pub const JACK_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Jack};
pub const QUEEN_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Queen};
pub const KING_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: King};
pub const ACE_DIAMONDS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Diamonds, figure: Ace};

pub const TWO_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(figure::F2)};
pub const THREE_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(figure::F3)};
pub const FOUR_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(figure::F4)};
pub const FIVE_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(figure::F5)};
pub const SIX_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(figure::F6)};
pub const SEVEN_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(figure::F7)};
pub const EIGHT_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(figure::F8)};
pub const NINE_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(figure::F9)};
pub const TEN_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Numbered(figure::F10)};
pub const JACK_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Jack};
pub const QUEEN_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Queen};
pub const KING_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: King};
pub const ACE_HEARTS: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Hearts, figure: Ace};

pub const TWO_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(figure::F2)};
pub const THREE_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(figure::F3)};
pub const FOUR_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(figure::F4)};
pub const FIVE_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(figure::F5)};
pub const SIX_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(figure::F6)};
pub const SEVEN_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(figure::F7)};
pub const EIGHT_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(figure::F8)};
pub const NINE_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(figure::F9)};
pub const TEN_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Numbered(figure::F10)};
pub const JACK_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Jack};
pub const QUEEN_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Queen};
pub const KING_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: King};
pub const ACE_SPADES: Card<FigureStd, SuitStd> = Card { suit: SuitStd::Spades, figure: Ace};
