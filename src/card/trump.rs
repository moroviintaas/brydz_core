use std::cmp::Ordering;
use crate::card::{Card};
use crate::card::figure::{Figure};
use crate::card::suit::{Suit, SuitStd};
use crate::card::suit::SuitStd::{Clubs, Diamonds, Hearts, Spades};
use crate::card::trump::Trump::{Colored, NoTrump};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Trump<S: Suit>{
    Colored(S),
    NoTrump
}
impl<S: Suit> Trump<S>{
    pub fn order_cards<F: Figure> (&self, card_one: &Card<F, S>, card_two: &Card<F, S>) -> Ordering{
        match self{
            Trump::NoTrump => {
                card_one.figure().cmp(&card_two.figure())
                    .then_with(|| card_one.suit().cmp(&card_two.suit()))
            },
            Trump::Colored(trump_suit) =>{
                match card_one.suit(){
                    equal if equal == card_two.suit() =>
                        card_one.figure().cmp(&card_two.figure()),
                    trumped if &trumped == &trump_suit => Ordering::Greater,
                    suit_one => match card_two.suit(){
                        trumped if &trumped == &trump_suit => Ordering::Less,
                        suit_two => card_one.figure().cmp(&card_two.figure())
                            .then_with(|| suit_one.cmp(&suit_two))
                    }
                }
            }
        }
    }
    /*pub fn order_rel_cards(&self, card_one: &ReleasedCard, card_two: &ReleasedCard) -> Ordering{
        self.order_cards(&card_one.card(), &card_two.card())
    }*/

    /*fn ord_value(&self) -> u8{
        match self{
            NoTrump  => 5,
            Colored(Spades) => 4,
            Colored(Hearts) => 3,
            Colored(Diamonds) => 2,
            Colored(Clubs) => 1
        }
    }*/



    /*
    !!! Nie działa
    /// Picks the highest card in slice according to `Trump`
    /// ```
    /// use arrayvec::ArrayVec;
    /// use bridge_core::card::suit::Suit;
    /// use bridge_core::card::suit::Suit::Hearts;
    /// use bridge_core::card::figure::Figure::Ace;
    /// use bridge_core::card::trump::Trump;
    /// use bridge_core::card::Card;
    /// use bridge_core::deck::Deck;
    /// use bridge_core::deck::DECK_SIZE;
    /// use std::boxed::Box;
    /// let trump1 = Trump::Colored(Hearts);
    ///
    /// let deck:Vec<Card>= Deck::new_id_rand().card().as_slice().iter().map(|rc| rc.card()).collect();
    ///
    /// assert_eq!(trump1.highest_in(Box::new(deck)).unwrap(), Card::new(Ace, Hearts));
    ///
    /// ```

    pub fn highest_in(&self, mut card: Box<dyn  IntoIterator<Item = Card, IntoIter = dyn Iterator<Item = Card>>>) -> Option<Card>{

        let mut highest_so_far = card.nth(0)?;
        for c in card {
            if self.order_cards(&c, &highest_so_far) == Ordering::Greater{
                highest_so_far = c;
            }
        }
        Some(highest_so_far)


    }
    */


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
    use crate::card::Card;
    use crate::card::figure::FigureStd::{Ace, Numbered, Queen};
    use crate::card::figure::NumberFigureStd;
    use crate::card::suit::SuitStd::{Diamonds, Hearts, Spades};
    use crate::card::trump::Trump;

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

