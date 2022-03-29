use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use crate::cards::{Card};
use serde::{Deserialize, Serialize};
use crate::cards::suit::Suit;
use crate::cards::trump::Trump;
use crate::play::exhaust::ExhaustTable;

use crate::play::trick::TrickError::{CardSlotAlreadyUsed, MissingCard, ViolatedOrder};

use crate::player::side::Side::{North, South, East, West};
use crate::player::side::{Side, SIDES};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum TrickError{
    MissingCard(Side),
    CardSlotAlreadyUsed(Side),
    DuplicateCard(Card),
    ViolatedOrder(Side)
}
impl Display for TrickError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Trick{
    north_card: Option<Card>,
    west_card: Option<Card>,
    south_card: Option<Card>,
    east_card: Option<Card>,
    first_player: Side,
    card_num: u8,

}

impl Index<Side> for Trick{
    type Output = Option<Card>;

    fn index(&self, index: Side ) -> &Self::Output {
        match index{
            North => &self.north_card,
            South => &self.south_card,
            West => &self.west_card,
            East => &self.east_card
        }
    }


}



impl IndexMut<Side> for Trick{
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index{
            Side::North => &mut self.north_card,
            Side::South => &mut self.south_card,
            Side::West => &mut self.west_card,
            Side::East => &mut self.east_card
        }
    }
}

impl Trick{
    pub fn new( first_player: Side) -> Self{

        Self{first_player, north_card: None, south_card: None, west_card: None, east_card: None, card_num: 0}
    }



    pub fn add_card(&mut self, side: Side, card: Card) -> Result<(), TrickError>{
        let side_in_order = self.first_player.next_i(self.card_num);
        match side == side_in_order{
            true => match self[side]{
                None => match self.contains(&card){
                    false => {
                        self.card_num += 1;
                        self[side] = Some(card);
                        Ok(())
                    }
                    true => Err(TrickError::DuplicateCard(card))
                }

                Some(_) => Err(CardSlotAlreadyUsed(side))
            },
            false => Err(ViolatedOrder(side_in_order))
        }
    }

    /// Checks if trick contains a  specific card
    /// ```
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::cards::Card;
    /// use bridge_core::cards::figure::Figure;
    /// use bridge_core::cards::suit::Suit;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::player::side::Side::North;
    ///
    /// let mut trick = Trick::new(North);
    /// trick.add_card(Side::North, Card::new(Figure::Jack, Suit::Spades));
    /// assert!(trick.contains(&Card::new(Figure::Jack, Suit::Spades)));
    /// assert!(!trick.contains(&Card::new(Figure::Ace, Suit::Spades)));
    /// ```
    pub fn contains(&self, card: &Card) -> bool{
        for side in [North, East, South, West]{
            if self[side].map_or(false, |c| c == *card){
                return true;
            }
        }
        false
    }


    /// Checks if two tricks collide in some card
    pub fn collides(&self, other: &Trick) -> bool{
        self.collision(other).is_some()


    }

    /// Checks if two tricks collide with some card
    /// # Returns:
    /// `Some(c: Card)` if there is a collision with card `c`
    /// `None` if there is no collision
    /// ```
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::cards::Card;
    /// use bridge_core::cards::figure::Figure;
    /// use bridge_core::cards::suit::Suit;
    /// use bridge_core::player::side::Side;
    ///
    /// let mut trick1 = Trick::new(Side::North);
    /// trick1.add_card(Side::North, Card::new(Figure::Jack, Suit::Spades)).unwrap();
    /// trick1.add_card(Side::East, Card::new(Figure::Ace, Suit::Spades)).unwrap();
    /// trick1.add_card(Side::South, Card::new(Figure::Ace, Suit::Hearts)).unwrap();
    /// let mut trick2 = Trick::new(Side::North);
    /// trick2.add_card(Side::North, Card::new(Figure::Jack, Suit::Hearts)).unwrap();
    /// trick2.add_card(Side::East, Card::new(Figure::Ace, Suit::Diamonds)).unwrap();
    /// assert_eq!(trick1.collision(&trick2), None);
    /// trick2.add_card(Side::South, Card::new(Figure::Ace, Suit::Hearts)).unwrap();
    /// assert_eq!(trick1.collision(&trick2), Some(Card::new(Figure::Ace, Suit::Hearts)));
    /// ```
    pub fn collision(&self, other: &Trick) -> Option<Card>{
        for oc in &[other[North], other[East], other[South], other[West]]{
            match oc {
                Some(c) => match self.contains(c){
                    true => {return Some(*c)},
                    false => {}
                },
                None => {}
            }
        }
        None
    }

    /// Checks if trick is complete
    ///
    /// ```
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::cards::Card;
    /// use bridge_core::cards::figure::Figure;
    /// use bridge_core::cards::suit::Suit;
    /// use bridge_core::player::side::Side;
    ///
    /// let mut trick = Trick::new(Side::North);
    /// trick.add_card(Side::North, Card::new(Figure::Jack, Suit::Spades));
    /// trick.add_card(Side::East, Card::new(Figure::Ace, Suit::Spades));
    /// trick.add_card(Side::South, Card::new(Figure::Ace, Suit::Hearts));
    /// assert!(!trick.is_complete());
    /// trick.add_card(Side::West, Card::new(Figure::Jack, Suit::Hearts));
    /// assert!(trick.is_complete());
    ///
    /// ```
    pub fn is_complete(&self) -> bool{

        self[North].and(self[East]).and(self[South]).and(self[West]).is_some()
    }
    pub fn missing_card(&self) -> Option<Side>{
        for s in SIDES{
            if self[s] == None{
                return Some(s)
            }
        }
        None
    }

    fn winner_of_2(&self, winner_so_far: Side, check_side: Side, trump: Trump) -> Result<Side, TrickError>{
        match self[check_side] {
            None => Err(MissingCard(check_side)),
            Some(_) => match trump.order_cards(&self[check_side].unwrap(), &self[winner_so_far].unwrap()) {
                Ordering::Greater => Ok(check_side),
                _ => Ok(winner_so_far)
            }
        }
    }

    /// Tries to pick a winner of a trick
    /// ```
    /// use bridge_core::cards::figure::Figure::*;
    /// use bridge_core::cards::figure::NumberFigure;
    /// use bridge_core::cards::suit::Suit::{*};
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::cards::trump::Trump::{Colored, NoTrump};
    /// use bridge_core::play::deck::Deck;
    /// use bridge_core::cards::Card;
    /// use bridge_core::player::role::PlayRole::{Declarer, Dummy, FirstDefender, SecondDefender};
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::player::side::Side::{North, South, East, West};
    /// use std::str::FromStr;
    /// let mut trick1 = Trick::new(North);
    ///
    /// let qh = Card::new(Queen, Hearts);
    /// let ap = Card::new(Ace, Spades);
    /// let tens = Card::new(Numbered(NumberFigure::new(10)), Spades);
    /// let twoc = Card::new(Numbered(NumberFigure::new(2)), Clubs);
    ///
    ///
    /// trick1.add_card(North, qh).unwrap();
    /// trick1.add_card(East, twoc).unwrap();
    /// trick1.add_card(South, ap).unwrap();
    /// trick1.add_card(West, tens).unwrap();
    ///
    ///
    /// assert_eq!(trick1.taker(Colored(Hearts)).unwrap(), North);
    /// let mut trick2 = Trick::new(North);
    ///
    ///
    /// trick2.add_card(North, qh).unwrap();
    /// trick2.add_card(East, twoc).unwrap();
    /// trick2.add_card(South, ap).unwrap();
    /// trick2.add_card(West, tens).unwrap();
    ///
    ///
    /// assert_eq!(trick2.taker(Colored(Clubs)).unwrap(), East);
    ///
    /// let mut trick3 = Trick::new(East);
    ///
    /// trick3.add_card(East, Card::from_str("a hearts").unwrap()).unwrap();
    /// trick3.add_card(South, ap).unwrap();
    /// trick3.add_card(West, tens).unwrap();
    /// trick3.add_card(North, qh).unwrap();
    /// assert_eq!(trick3.taker(NoTrump).unwrap(), East);
    ///
    ///
    /// ```
    pub fn taker(&self, trump: Trump) -> Result<Side, TrickError>{
        let mut winner_so_far = match self.north_card {
            None => { return Err(MissingCard(North))},
            Some(_) => North
        };

        match trump{
            Trump::Colored(_) => {
                winner_so_far = self.winner_of_2(winner_so_far, South, trump)?;
                winner_so_far = self.winner_of_2(winner_so_far, West, trump)?;
                winner_so_far = self.winner_of_2(winner_so_far, East, trump)?;
                Ok(winner_so_far)
            },
            Trump::NoTrump => {
                match self[self.first_player]{
                    None => Err(MissingCard(self.first_player)),
                    Some(s) => {
                        let tmp_trump = Trump::Colored(s.suit());
                        winner_so_far = self.winner_of_2(winner_so_far, South, tmp_trump)?;
                        winner_so_far = self.winner_of_2(winner_so_far, West, tmp_trump)?;
                        winner_so_far = self.winner_of_2(winner_so_far, East, tmp_trump)?;
                        Ok(winner_so_far)
                    }

                }
            }
        }

    }
    pub fn prepare_new(&self, trump: Trump) -> Option<Self>{
        self.taker(trump).ok().map(|s| Trick::new(s))
    }
    pub fn called_suit(&self) -> Option<Suit>{
        self[self.first_player].map(|c| c.suit())
    }
    pub fn first_player_side(&self) -> Side{
        self.first_player
    }

}
/*
impl Default for Trick{
    fn default() -> Self {
        Self::new(South)
    }
}
*/