use std::cmp::Ordering;
use crate::cards::{Card, suit};
use serde::{Deserialize, Serialize};
use crate::cards::suit::Suit::{Clubs, Diamonds, Hearts, Spades};
use crate::cards::trump::Trump::{Colored, NoTrump};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum Trump{
    Colored(suit::Suit),
    NoTrump
}
impl Trump{
    pub fn order_cards(&self, card_one: &Card, card_two: &Card) -> Ordering{
        match self{
            Trump::NoTrump => {
                card_one.figure().cmp(&card_two.figure())
                    .then_with(|| card_one.suit().cmp(&card_two.suit()))
            },
            Trump::Colored(trump_suit) =>{
                match card_one.suit(){
                    equal if equal == card_two.suit() =>
                        card_one.figure().cmp(&card_two.figure()),
                    trumped if &trumped == trump_suit => Ordering::Greater,
                    suit_one => match card_two.suit(){
                        trumped if &trumped == trump_suit => Ordering::Less,
                        suit_two => card_one.figure().cmp(&card_two.figure())
                            .then_with(|| suit_one.cmp(&suit_two))
                    }
                }
            }
        }
    }
    /*pub fn order_rel_cards(&self, card_one: &ReleasedCard, card_two: &ReleasedCard) -> Ordering{
        self.order_cards(&card_one.cards(), &card_two.cards())
    }*/

    fn ord_value(&self) -> u8{
        match self{
            NoTrump  => 5,
            Colored(Spades) => 4,
            Colored(Hearts) => 3,
            Colored(Diamonds) => 2,
            Colored(Clubs) => 1
        }
    }



    /*
    !!! Nie działa
    /// Picks the highest cards in slice according to `Trump`
    /// ```
    /// use arrayvec::ArrayVec;
    /// use bridge_core::cards::suit::Suit;
    /// use bridge_core::cards::suit::Suit::Hearts;
    /// use bridge_core::cards::figure::Figure::Ace;
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::cards::Card;
    /// use bridge_core::deck::Deck;
    /// use bridge_core::deck::DECK_SIZE;
    /// use std::boxed::Box;
    /// let trump1 = Trump::Colored(Hearts);
    ///
    /// let deck:Vec<Card>= Deck::new_id_rand().cards().as_slice().iter().map(|rc| rc.cards()).collect();
    ///
    /// assert_eq!(trump1.highest_in(Box::new(deck)).unwrap(), Card::new(Ace, Hearts));
    ///
    /// ```

    pub fn highest_in(&self, mut cards: Box<dyn  IntoIterator<Item = Card, IntoIter = dyn Iterator<Item = Card>>>) -> Option<Card>{

        let mut highest_so_far = cards.nth(0)?;
        for c in cards {
            if self.order_cards(&c, &highest_so_far) == Ordering::Greater{
                highest_so_far = c;
            }
        }
        Some(highest_so_far)


    }
    */


}

impl PartialOrd for Trump{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.ord_value().cmp(&other.ord_value()))
    }
}

impl Ord for Trump{
    fn cmp(&self, other: &Self) -> Ordering {
        self.ord_value().cmp(&other.ord_value())
    }
}

pub const TRUMPS: [Trump; 5] = [Colored(Spades), Colored(Hearts), Colored(Diamonds), Colored(Clubs), NoTrump];

#[cfg(test)]
mod tests{
    use std::cmp::Ordering;
    use crate::cards::Card;
    use crate::cards::figure::Figure::{Ace, Numbered, Queen};
    use crate::cards::figure::NumberFigure;
    use crate::cards::suit::Suit::{Diamonds, Hearts, Spades};
    use crate::cards::trump::Trump;

    #[test]
    fn trump_diamonds(){
        let c1 = Card::new(Ace, Spades);
        let c2 = Card::new(Numbered(NumberFigure::new(10)), Diamonds);
        let c3 = Card::new(Queen, Diamonds);
        let c4 = Card::new(Numbered(NumberFigure::new(4)), Spades);
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
        let c2 = Card::new(Numbered(NumberFigure::new(10)), Diamonds);
        let c3 = Card::new(Queen, Diamonds);
        let c4 = Card::new(Numbered(NumberFigure::new(4)), Spades);
        let c5 = Card::new(Ace, Hearts);
        let trump = Trump::NoTrump;

        assert_eq!(trump.order_cards(&c1, &c2), Ordering::Greater);
        assert_eq!(trump.order_cards(&c2, &c3), Ordering::Less);
        assert_eq!(trump.order_cards(&c1, &c4), Ordering::Greater);
        assert_eq!(trump.order_cards(&c1, &c5), Ordering::Greater);

    }
}

