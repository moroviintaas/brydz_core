use std::cmp::Ordering;
use std::ops::{Index, IndexMut};
use karty::cards::{Card2SymTrait, Card};
use karty::register::Register;
use crate::cards::trump::Trump;

use crate::error::TrickError::{CardSlotAlreadyUsed, MissingCard, ViolatedOrder};
use crate::error::{Mismatch, TrickError};

use crate::player::side::Side::{North, South, East, West};
use crate::player::side::{Side, SIDES};





#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Trick<Card: Card2SymTrait>{
    north_card: Option<Card>,
    west_card: Option<Card>,
    south_card: Option<Card>,
    east_card: Option<Card>,
    first_player: Side,
    card_num: u8,

}

pub type TrickStd = Trick<Card>;

impl<Card: Card2SymTrait + Copy> Copy for Trick<Card>{}

impl<Card: Card2SymTrait> Index<Side> for Trick<Card>{
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



impl<Card: Card2SymTrait> IndexMut<Side> for Trick<Card>{
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index{
            Side::North => &mut self.north_card,
            Side::South => &mut self.south_card,
            Side::West => &mut self.west_card,
            Side::East => &mut self.east_card
        }
    }
}

impl<Card: Card2SymTrait> Trick<Card>{
    pub fn new( first_player: Side) -> Self{

        Self{first_player, north_card: None, south_card: None, west_card: None, east_card: None, card_num: 0}
    }

    /// # Returns:
    /// Option of whose turn it is now
    ///
    /// `Some(Side)` if determined
    /// `None` if trick is completed
    /// ```
    /// use brydz_core::contract::Trick;
    /// use brydz_core::player::side::Side::{East, North, South, West};
    /// use karty::cards::{ACE_SPADES, KING_CLUBS, KING_DIAMONDS, KING_HEARTS};
    /// let mut trick = Trick::new(North);
    /// assert_eq!(trick.current_side(), Some(North));
    /// trick.add_card_no_register(North, ACE_SPADES).unwrap();
    /// assert_eq!(trick.current_side(), Some(East));
    /// trick.add_card_no_register(East, KING_HEARTS).unwrap();
    /// assert_eq!(trick.current_side(), Some(South));
    /// trick.add_card_no_register(South, KING_DIAMONDS).unwrap();
    /// assert_eq!(trick.current_side(), Some(West));
    /// trick.add_card_no_register(West, KING_CLUBS).unwrap();
    /// assert!(trick.current_side().is_none());
    /// ```
    pub fn current_side(&self) -> Option<Side>{
        match self.card_num{
            x@ 0..=3 => Some(self.first_player.next_i(x)),
            _ => None
        }
    }




    /// Adds card to trick with support for checking and updating suit exhaust table
    /// # Examples
    /// ```
    /// use brydz_core::contract::collision::SuitExhaustStd;
    /// use brydz_core::player::side::Side;
    /// use brydz_core::error::TrickError;
    /// use brydz_core::contract::Trick;
    /// use std::str::FromStr;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::{CardRegister, Register};
    /// use karty::cards::*;
    ///
    /// let mut exhaust_table = SuitExhaustStd::default();
    /// let mut trick1 = Trick::<Card>::new(Side::West);
    /// trick1.add_card(Side::West, JACK_CLUBS, &mut exhaust_table).unwrap();
    /// let r1 = trick1.add_card(Side::North, TEN_CLUBS, &mut exhaust_table);
    /// assert_eq!(r1, Ok(2));
    /// let r2 = trick1.add_card(Side::East, NINE_HEARTS, &mut exhaust_table);
    /// assert_eq!(r2, Ok(3));
    /// assert!(exhaust_table.is_registered(&(Side::East, Suit::Clubs)));
    /// let mut trick2 = Trick::new(Side::East);
    /// let r3 = trick2.add_card(Side::East, NINE_CLUBS, &mut exhaust_table);
    /// assert_eq!(r3, Err(TrickError::UsedPreviouslyExhaustedSuit(Suit::Clubs)));
    ///
    /// ```
    pub fn add_card<Se: Register<(Side, Card::Suit)>>(&mut self, side: Side, card: Card, exhaust_register: &mut Se) -> Result<u8, TrickError<Card>>{
        //if exhaust_register.is_exhausted(&side, card.suit()){
        if exhaust_register.is_registered(&(side, card.suit().to_owned())){
            // This suit was already exhausted for player, therefore possible cheating
            return Err(TrickError::UsedPreviouslyExhaustedSuit(card.suit().to_owned()))
        }
        let side_in_order = match self.current_side(){
            Some(s) => s,
            None => { return Err(TrickError::TrickFull)}
        };
        //let side_in_order = self.first_player.next_i(self.card_num);
        match side == side_in_order{
            true => match self[side]{
                None => match self.contains(&card){
                    false => {
                        if side != self.first_player && card.suit() != self[self.first_player].as_ref().unwrap().suit() {
                            // mark suit of first card in trick as exhausted for the player
                            //exhaust_register.mark_exhausted(&side, self[self.first_player].as_ref().unwrap().suit())
                            exhaust_register.register((side, self[self.first_player].as_ref().unwrap().suit().to_owned()))
                        }
                        self.card_num += 1;
                        self[side] = Some(card);
                        Ok(self.card_num)
                    }
                    true => Err(TrickError::DuplicateCard(card))
                }

                Some(_) => Err(CardSlotAlreadyUsed(side))
            },
            false => Err(ViolatedOrder(Mismatch{expected:side_in_order, found: side}))
        }
    }

    pub fn add_card_no_register(&mut self, side: Side, card: Card) ->  Result<u8, TrickError<Card>>{
        let side_in_order = match self.current_side(){
            Some(s) => s,
            None => { return Err(TrickError::TrickFull)}
        };
        match side == side_in_order{
            true => match self[side]{
                None => match self.contains(&card){
                    false => {
                        self.card_num += 1;
                        self[side] = Some(card);
                        Ok(self.card_num)
                    }
                    true => Err(TrickError::DuplicateCard(card))
                }

                Some(_) => Err(CardSlotAlreadyUsed(side))
            },
            false => Err(ViolatedOrder(Mismatch{expected:side_in_order, found: side}))
        }
    }





    /// Checks if trick contains a  specific card
    /// ```
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::contract::Trick;
    /// use brydz_core::player::side::Side;
    /// use brydz_core::contract::collision::{SuitExhaustStd};
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut exhaust_register = SuitExhaustStd::default();
    ///
    /// let mut trick = Trick::new(Side::North);
    /// trick.add_card(Side::North, JACK_SPADES, &mut exhaust_register);
    /// assert!(trick.contains(&JACK_SPADES));
    /// assert!(!trick.contains(&ACE_SPADES));
    /// ```
    pub fn contains(&self, card: &Card) -> bool{
        for side in [North, East, South, West]{
            if self[side].as_ref().map_or(false, |c| c == card){
                return true;
            }
        }
        false
    }


    /// Checks if two tricks collide in some card
    pub fn collides(&self, other: &Trick<Card>) -> bool{
        self.collision(other).is_some()
    }



    /// Checks if two tricks collide with some card
    /// # Returns:
    /// `Some(c: Card)` if there is a collision with card `c`
    /// `None` if there is no collision
    /// ```
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::contract::Trick;
    /// use brydz_core::player::side::Side;
    /// use brydz_core::contract::collision::SuitExhaustStd;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut trick1 = Trick::new(Side::North);
    /// let mut exhaust_register = SuitExhaustStd::default();
    /// trick1.add_card(Side::North, JACK_SPADES,&mut exhaust_register).unwrap();
    ///
    /// trick1.add_card(Side::East, ACE_SPADES, &mut exhaust_register).unwrap();
    /// trick1.add_card(Side::South, ACE_HEARTS, &mut exhaust_register).unwrap();
    /// let mut trick2 = Trick::new(Side::North, );
    /// trick2.add_card(Side::North, JACK_HEARTS, &mut exhaust_register).unwrap();
    /// trick2.add_card(Side::East, ACE_DIAMONDS, &mut exhaust_register).unwrap();
    /// assert_eq!(trick1.collision(&trick2), None);
    /// trick2.add_card(Side::South, ACE_HEARTS, &mut exhaust_register).unwrap();
    /// assert_eq!(trick1.collision(&trick2), Some(ACE_HEARTS));
    /// ```
    pub fn collision(&self, other: &Trick<Card>) -> Option<Card>{
        for oc in [&other[North], &other[East], &other[South], &other[West]]{
            match oc {
                Some(c) => match self.contains(c){
                    true => {return Some(c.to_owned())},
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
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::contract::Trick;
    /// use brydz_core::player::side::Side;
    /// use brydz_core::contract::collision::SuitExhaustStd;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut exhaust_register = SuitExhaustStd::default();
    /// let mut trick = Trick::new(Side::North);
    /// trick.add_card(Side::North, JACK_SPADES, &mut exhaust_register);
    /// trick.add_card(Side::East, ACE_SPADES, &mut exhaust_register);
    /// trick.add_card(Side::South, ACE_HEARTS, &mut exhaust_register);
    /// assert!(!trick.is_complete());
    /// trick.add_card(Side::West, JACK_HEARTS, &mut exhaust_register);
    /// assert!(trick.is_complete());
    ///
    /// ```
    pub fn is_complete(&self) -> bool{

        self[North].as_ref()
            .and(self[East].as_ref())
            .and(self[South].as_ref())
            .and(self[West].as_ref())
            .is_some()
    }
    pub fn is_empty(&self) -> bool {
        self[North].as_ref()
            .or(self[East].as_ref())
            .or(self[South].as_ref())
            .or(self[West].as_ref())
            .is_none()
    }
    pub fn missing_card(&self) -> Option<Side>{
        for s in SIDES{
            if self[s] == None{
                return Some(s)
            }
        }
        None
    }

    fn winner_of_2(&self, winner_so_far: Side, check_side: Side, trump: &Trump<Card::Suit>) -> Result<Side, TrickError<Card>>{
        match self[check_side] {
            None => Err(MissingCard(check_side)),
            Some(_) => match trump.order_cards(self[check_side].as_ref().unwrap(), self[winner_so_far].as_ref().unwrap()) {
                Ordering::Greater => Ok(check_side),
                _ => Ok(winner_so_far)
            }
        }
    }

    /// Tries to pick a winner of a trick
    /// ```
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::cards::trump::Trump::{Colored, NoTrump};
    /// use brydz_core::cards::deck::Deck;
    /// use brydz_core::player::role::PlayRole::{Declarer, Dummy, FirstDefender, SecondDefender};
    /// use brydz_core::contract::Trick;
    /// use brydz_core::player::side::Side::{North, South, East, West};
    /// use std::str::FromStr;
    /// use brydz_core::contract::collision::SuitExhaustStd;
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut exhaust_register = SuitExhaustStd::default();
    /// let mut trick1 = Trick::new(North);
    /// trick1.add_card(North, QUEEN_HEARTS, &mut exhaust_register).unwrap();
    /// trick1.add_card(East, TWO_CLUBS, &mut exhaust_register).unwrap();
    /// trick1.add_card(South, ACE_SPADES, &mut exhaust_register).unwrap();
    /// trick1.add_card(West, TEN_SPADES, &mut exhaust_register).unwrap();
    /// assert_eq!(trick1.taker(&Colored(Hearts)).unwrap(), North);
    /// let mut trick2 = Trick::new(North);
    ///
    /// trick2.add_card(North, QUEEN_HEARTS, &mut exhaust_register).unwrap();
    /// trick2.add_card(East, TWO_CLUBS, &mut exhaust_register).unwrap();
    /// trick2.add_card(South, ACE_SPADES, &mut exhaust_register).unwrap();
    /// trick2.add_card(West, TEN_SPADES, &mut exhaust_register).unwrap();
    /// assert_eq!(trick2.taker(&Colored(Clubs)).unwrap(), East);
    ///
    /// let mut trick3 = Trick::new(East);
    /// trick3.add_card(East, ACE_CLUBS, &mut exhaust_register).unwrap();
    /// trick3.add_card(South, ACE_SPADES, &mut exhaust_register).unwrap();
    /// trick3.add_card(West, TEN_SPADES, &mut exhaust_register).unwrap();
    /// trick3.add_card(North, QUEEN_HEARTS, &mut exhaust_register).unwrap();
    /// assert_eq!(trick3.taker(&NoTrump).unwrap(), East);
    /// ```
    pub fn taker(&self, trump: &Trump<Card::Suit>) -> Result<Side, TrickError<Card>>{
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
                match &self[self.first_player]{
                    None => Err(MissingCard(self.first_player)),
                    Some(s) => {
                        let tmp_trump = Trump::Colored(s.suit().clone());
                        winner_so_far = self.winner_of_2(winner_so_far, South, &tmp_trump)?;
                        winner_so_far = self.winner_of_2(winner_so_far, West, &tmp_trump)?;
                        winner_so_far = self.winner_of_2(winner_so_far, East, &tmp_trump)?;
                        Ok(winner_so_far)
                    }

                }
            }
        }

    }
    pub fn prepare_new(&self, trump: Trump<Card::Suit>) -> Option<Self>{
        self.taker(&trump).ok().map(|s| Trick::new(s))
    }
    pub fn called_suit(&self) -> Option<&Card::Suit>{
        self[self.first_player].as_ref().map(|c| c.suit())
    }
    pub fn first_player_side(&self) -> Side{
        self.first_player
    }

}

impl<Card: Card2SymTrait> Default for Trick<Card>{
    fn default() -> Self {
        Self{card_num:0, first_player: North, north_card: None, east_card: None, south_card: None, west_card:None}
    }
}