use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use crate::card::{Card};
use serde::{Deserialize, Serialize};
use crate::card::suit::Suit;
use crate::card::trump::Trump;
use crate::play::exhaust::{SuitExhaust};

use crate::play::trick::TrickError::{CardSlotAlreadyUsed, MissingCard, ViolatedOrder};

use crate::player::side::Side::{North, South, East, West};
use crate::player::side::{Side, SIDES};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum TrickError{
    MissingCard(Side),
    CardSlotAlreadyUsed(Side),
    DuplicateCard(Card),
    ViolatedOrder(Side),
    UsedPreviouslyExhaustedSuit(Suit),
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

    /// Adds card to trick with support for checking and updating suit exhaust table
    /// # Examples
    /// ```
    /// use bridge_core::card::Card;
    /// use bridge_core::play::exhaust::SuitExhaust;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::play::trick::{Trick, TrickError};
    /// use std::str::FromStr;
    /// use bridge_core::card::suit::Suit;
    /// use bridge_core::card;
    /// let mut exhaust_table = SuitExhaust::new();
    /// let mut trick1 = Trick::new(Side::West);
    /// trick1.add_card_check_exhaust(Side::West, card::JACK_CLUBS, &mut exhaust_table).unwrap();
    /// let r1 = trick1.add_card_check_exhaust(Side::North, card::TEN_CLUBS, &mut exhaust_table);
    /// assert_eq!(r1, Ok(2));
    /// let r2 = trick1.add_card_check_exhaust(Side::East, card::NINE_HEARTS, &mut exhaust_table);
    /// assert_eq!(r2, Ok(3));
    /// assert!(exhaust_table.get_exhaust(Side::East, Suit::Clubs));
    /// let mut trick2 = Trick::new(Side::East);
    /// let r3 = trick2.add_card_check_exhaust(Side::East, card::NINE_CLUBS, &mut exhaust_table);
    /// assert_eq!(r3, Err(TrickError::UsedPreviouslyExhaustedSuit(Suit::Clubs)));
    ///
    /// ```
    pub fn add_card_check_exhaust(&mut self, side: Side, card: Card, exhaust_table: &mut SuitExhaust) -> Result<u8, TrickError>{
        if exhaust_table.get_exhaust(side, card.suit()){
            // This suit was already exhausted for player, therefore possible cheating
            return Err(TrickError::UsedPreviouslyExhaustedSuit(card.suit()))
        }
        let side_in_order = self.first_player.next_i(self.card_num);
        match side == side_in_order{
            true => match self[side]{
                None => match self.contains(&card){
                    false => {
                        if side != self.first_player{
                            if card.suit() != self[self.first_player].unwrap().suit() {
                                // mark suit of first card in trick as exhausted for the player
                                exhaust_table.exhaust(&side, &self[self.first_player].unwrap().suit())
                            }
                        }
                        self.card_num += 1;
                        self[side] = Some(card);
                        Ok(self.card_num)
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
    /// use bridge_core::card::trump::Trump;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::card::Card;
    /// use bridge_core::card::figure::Figure;
    /// use bridge_core::card::suit::Suit;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::card;
    ///
    /// let mut trick = Trick::new(Side::North);
    /// trick.add_card(Side::North, card::JACK_SPADES);
    /// assert!(trick.contains(&card::JACK_SPADES));
    /// assert!(!trick.contains(&card::ACE_SPADES));
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
    /// use bridge_core::card::trump::Trump;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::card::Card;
    /// use bridge_core::card::figure::Figure;
    /// use bridge_core::card::suit::Suit;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::card;
    ///
    /// let mut trick1 = Trick::new(Side::North);
    /// trick1.add_card(Side::North, card::JACK_SPADES).unwrap();
    /// trick1.add_card(Side::East, card::ACE_SPADES).unwrap();
    /// trick1.add_card(Side::South, card::ACE_HEARTS).unwrap();
    /// let mut trick2 = Trick::new(Side::North);
    /// trick2.add_card(Side::North, card::JACK_HEARTS).unwrap();
    /// trick2.add_card(Side::East, card::ACE_DIAMONDS).unwrap();
    /// assert_eq!(trick1.collision(&trick2), None);
    /// trick2.add_card(Side::South, card::ACE_HEARTS).unwrap();
    /// assert_eq!(trick1.collision(&trick2), Some(card::ACE_HEARTS));
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
    /// use bridge_core::card::trump::Trump;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::card::Card;
    /// use bridge_core::card::figure::Figure;
    /// use bridge_core::card::suit::Suit;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::card;
    ///
    /// let mut trick = Trick::new(Side::North);
    /// trick.add_card(Side::North, card::JACK_SPADES);
    /// trick.add_card(Side::East, card::ACE_SPADES);
    /// trick.add_card(Side::South, card::ACE_HEARTS);
    /// assert!(!trick.is_complete());
    /// trick.add_card(Side::West, card::JACK_HEARTS);
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
    /// use bridge_core::card::figure::Figure::*;
    /// use bridge_core::card::figure::NumberFigure;
    /// use bridge_core::card::suit::Suit::{*};
    /// use bridge_core::card::trump::Trump;
    /// use bridge_core::card::trump::Trump::{Colored, NoTrump};
    /// use bridge_core::play::deck::Deck;
    /// use bridge_core::card::Card;
    /// use bridge_core::player::role::PlayRole::{Declarer, Dummy, FirstDefender, SecondDefender};
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::player::side::Side::{North, South, East, West};
    /// use bridge_core::card;
    /// use std::str::FromStr;
    /// let mut trick1 = Trick::new(North);
    ///
    /// trick1.add_card(North, card::QUEEN_HEARTS).unwrap();
    /// trick1.add_card(East, card::TWO_CLUBS).unwrap();
    /// trick1.add_card(South, card::ACE_SPADES).unwrap();
    /// trick1.add_card(West, card::TEN_SPADES).unwrap();
    /// assert_eq!(trick1.taker(Colored(Hearts)).unwrap(), North);
    /// let mut trick2 = Trick::new(North);
    ///
    /// trick2.add_card(North, card::QUEEN_HEARTS).unwrap();
    /// trick2.add_card(East, card::TWO_CLUBS).unwrap();
    /// trick2.add_card(South, card::ACE_SPADES).unwrap();
    /// trick2.add_card(West, card::TEN_SPADES).unwrap();
    /// assert_eq!(trick2.taker(Colored(Clubs)).unwrap(), East);
    ///
    /// let mut trick3 = Trick::new(East);
    /// trick3.add_card(East, card::ACE_HEARTS).unwrap();
    /// trick3.add_card(South, card::ACE_SPADES).unwrap();
    /// trick3.add_card(West, card::TEN_SPADES).unwrap();
    /// trick3.add_card(North, card::QUEEN_HEARTS).unwrap();
    /// assert_eq!(trick3.taker(NoTrump).unwrap(), East);
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