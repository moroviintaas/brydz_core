use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash};
use karty::cards::{Card2SymTrait};
use karty::suits::{SuitTrait, Suit};
use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use crate::cards::trump::Trump::{Colored, NoTrump};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Trump<S: SuitTrait>{
    Colored(S),
    NoTrump
}

pub type TrumpStd = Trump<Suit>;

impl<S: SuitTrait> Trump<S>{
    pub fn order_cards<Card: Card2SymTrait<Suit = S>> (&self, card_one: &Card, card_two: &Card) -> Ordering{
        match self{
            Trump::NoTrump => {
                card_one.figure().cmp(card_two.figure())
                    .then_with(|| card_one.suit().cmp(card_two.suit()))
            },
            Trump::Colored(trump_suit) =>{
                match card_one.suit(){
                    equal if equal == card_two.suit() =>
                        card_one.figure().cmp(card_two.figure()),
                    trumped if trumped == trump_suit => Ordering::Greater,
                    suit_one => match card_two.suit(){
                        trumped if trumped == trump_suit => Ordering::Less,
                        suit_two => card_one.figure().cmp(card_two.figure())
                            .then_with(|| suit_one.cmp(suit_two))
                    }
                }
            }
        }
    }



}


impl<S: SuitTrait> PartialOrd for Trump<S>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: SuitTrait> Ord for Trump<S>{
    fn cmp(&self, other: &Self) -> Ordering {
        match self{
            NoTrump => match other{
                NoTrump => Ordering::Equal,
                _ => Ordering::Greater
            },
            Colored(left) => match other {
                NoTrump => Ordering::Less,
                Colored(right) => left.cmp(right)
            }
        }
    }
}

impl <S: SuitTrait + Display> Display for Trump<S>{
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub const TRUMPS: [Trump<Suit>; 5] = [Colored(Spades), Colored(Hearts), Colored(Diamonds), Colored(Clubs), NoTrump];

#[cfg(test)]
mod tests{
    use std::cmp::Ordering;
    use karty::cards::Card2SGen;
    use karty::figures::{Ace, Numbered, NumberFigure, Queen};
    use karty::suits::Suit::{Diamonds, Hearts, Spades};

    use crate::cards::trump::Trump;

    #[test]
    fn trump_diamonds(){
        let c1 = Card2SGen::new(Ace, Spades);
        let c2 = Card2SGen::new(Numbered(NumberFigure::new(10)), Diamonds);
        let c3 = Card2SGen::new(Queen, Diamonds);
        let c4 = Card2SGen::new(Numbered(NumberFigure::new(4)), Spades);
        let c5 = Card2SGen::new(Ace, Hearts);
        let trump = Trump::Colored(Diamonds);

        assert_eq!(trump.order_cards(&c1, &c2), Ordering::Less);
        assert_eq!(trump.order_cards(&c2, &c3), Ordering::Less);
        assert_eq!(trump.order_cards(&c1, &c4), Ordering::Greater);
        assert_eq!(trump.order_cards(&c1, &c5), Ordering::Greater);

    }

    #[test]
    fn no_trump(){
        let c1 = Card2SGen::new(Ace, Spades);
        let c2 = Card2SGen::new(Numbered(NumberFigure::new(10)), Diamonds);
        let c3 = Card2SGen::new(Queen, Diamonds);
        let c4 = Card2SGen::new(Numbered(NumberFigure::new(4)), Spades);
        let c5 = Card2SGen::new(Ace, Hearts);
        let trump = Trump::NoTrump;

        assert_eq!(trump.order_cards(&c1, &c2), Ordering::Greater);
        assert_eq!(trump.order_cards(&c2, &c3), Ordering::Less);
        assert_eq!(trump.order_cards(&c1, &c4), Ordering::Greater);
        assert_eq!(trump.order_cards(&c1, &c5), Ordering::Greater);

    }
}

