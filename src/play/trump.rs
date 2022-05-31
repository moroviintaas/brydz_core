use std::cmp::Ordering;
use std::hash::{Hash};
use karty::cards::Card;
use karty::figures::Figure;
use karty::suits::{Suit, SuitStd};
use karty::suits::SuitStd::{Clubs, Diamonds, Hearts, Spades};

use crate::play::trump::Trump::{Colored, NoTrump};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Trump<S: Suit>{
    Colored(S),
    NoTrump
}
impl<S: Suit> Trump<S>{
    pub fn order_cards<F: Figure> (&self, card_one: &Card<F, S>, card_two: &Card<F, S>) -> Ordering{
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


impl<S:Suit> PartialOrd for Trump<S>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: Suit> Ord for Trump<S>{
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

pub const TRUMPS: [Trump<SuitStd>; 5] = [Colored(Spades), Colored(Hearts), Colored(Diamonds), Colored(Clubs), NoTrump];

#[cfg(test)]
mod tests{
    use std::cmp::Ordering;
    use karty::cards::Card;
    use karty::figures::{Ace, Numbered, NumberFigureStd, Queen};
    use karty::suits::SuitStd::{Diamonds, Hearts, Spades};

    use crate::play::trump::Trump;

    #[test]
    fn trump_diamonds(){
        let c1 = Card::new(Ace, Spades);
        let c2 = Card::new(Numbered(NumberFigureStd::new(10)), Diamonds);
        let c3 = Card::new(Queen, Diamonds);
        let c4 = Card::new(Numbered(NumberFigureStd::new(4)), Spades);
        let c5 = Card::new(Ace, Hearts);
        let trump = Trump::Colored(Diamonds);

        assert_eq!(trump.order_cards(&c1, &c2), Ordering::Less);
        assert_eq!(trump.order_cards(&c2, &c3), Ordering::Less);
        assert_eq!(trump.order_cards(&c1, &c4), Ordering::Greater);
        assert_eq!(trump.order_cards(&c1, &c5), Ordering::Greater);

    }

    #[test]
    fn no_trump(){
        let c1 = Card::new(Ace, Spades);
        let c2 = Card::new(Numbered(NumberFigureStd::new(10)), Diamonds);
        let c3 = Card::new(Queen, Diamonds);
        let c4 = Card::new(Numbered(NumberFigureStd::new(4)), Spades);
        let c5 = Card::new(Ace, Hearts);
        let trump = Trump::NoTrump;

        assert_eq!(trump.order_cards(&c1, &c2), Ordering::Greater);
        assert_eq!(trump.order_cards(&c2, &c3), Ordering::Less);
        assert_eq!(trump.order_cards(&c1, &c4), Ordering::Greater);
        assert_eq!(trump.order_cards(&c1, &c5), Ordering::Greater);

    }
}

