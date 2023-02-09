use std::fmt::{Debug, format};
use std::marker::PhantomData;
use karty::cards::Card2SymTrait;
use crate::contract::TrickGen;
use crate::error::TrickErrorGen;
use crate::error::TrickErrorGen::MissingCard;
use crate::player::side::Side;

pub trait TrickSolver: Debug + Clone{
    type CardType: Card2SymTrait;
    fn taker(&self, trick: &TrickGen<Self::CardType>) -> Result<Side, TrickErrorGen<Self::CardType>>;
    fn leader(&self, trick: &TrickGen<Self::CardType>) -> Option<Side>;
    fn does_beat_leader(&self, trick: &TrickGen<Self::CardType>, card: &Self::CardType) -> bool;
}


#[derive(Debug,  Clone, PartialEq, Eq)]
pub struct TrumpTrickSolver<Crd: Card2SymTrait>{
    trump_suit: <Crd as Card2SymTrait>::Suit,
}

impl <Crd: Card2SymTrait> TrumpTrickSolver<Crd>{
    pub fn new(trump_suit: <Crd as Card2SymTrait>::Suit) -> Self{
        Self{trump_suit}
    }
}

impl <Crd: Card2SymTrait> TrickSolver for TrumpTrickSolver<Crd>{
    type CardType = Crd;

    /// Tries to pick a winner of a trick
    /// ```
    ///
    /// use brydz_core::contract::{TrickGen, TrickSolver, TrumpTrickSolver};
    /// use brydz_core::error::TrickErrorGen::MissingCard;
    /// use brydz_core::player::side::Side::*;
    /// use karty::cards::*;
    /// use karty::suits::Suit::*;
    /// let mut trick1 = TrickGen::new(North);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, TWO_CLUBS).unwrap();
    /// trick1.insert_card(South, ACE_SPADES).unwrap();
    /// assert_eq!(TrumpTrickSolver::new(Hearts).taker(&trick1), Err(MissingCard(West)));
    /// trick1.insert_card(West, TEN_SPADES).unwrap();
    /// assert_eq!(TrumpTrickSolver::new(Hearts).taker(&trick1), Ok(North));
    /// assert_eq!(TrumpTrickSolver::new(Spades).taker(&trick1), Ok(South));
    /// ```
    fn taker(&self, trick: &TrickGen<Self::CardType>) -> Result<Side, TrickErrorGen<Self::CardType>> {
        match trick.is_complete(){
            false => Err(MissingCard(trick.first_player_side().next_i(trick.count_cards()))),
            true => {
                match trick.leader_in_suit(&self.trump_suit){
                    None => Ok(trick.leader_in_suit(
                        &trick.called_suit()
                            .unwrap_or_else(|| panic!("It is a bug: No leader selected in called suit {trick:?}"))
                        ).unwrap_or_else(|| panic!("It is a bug: Trick marked as completed, yet couldn't resolve called suit. Trick: {trick:?}"))),
                    Some(s) => Ok(s)
                }
            }
        }
    }
    /// ```
    /// use brydz_core::contract::{TrickGen, TrickSolver, TrumpTrickSolver};
    /// use brydz_core::player::side::Side::*;
    /// use karty::cards::*;
    /// use karty::suits::Suit::*;
    /// let mut trick1 = TrickGen::new(North);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, ACE_CLUBS).unwrap();
    /// trick1.insert_card(South, KING_HEARTS).unwrap();
    /// trick1.insert_card(West, TEN_SPADES).unwrap();
    /// assert_eq!(TrumpTrickSolver::new(Hearts).leader(&trick1), Some(South));
    /// assert_eq!(TrumpTrickSolver::new(Clubs).leader(&trick1), Some(East));
    /// assert_eq!(TrumpTrickSolver::new(Spades).leader(&trick1), Some(West));
    /// assert_eq!(TrumpTrickSolver::new(Diamonds).leader(&trick1), Some(South));
    /// ```
    fn leader(&self, trick: &TrickGen<Self::CardType>) -> Option<Side> {

        trick.leader_in_suit(&self.trump_suit).or_else(||{
            trick.called_suit().and_then(|s| trick.leader_in_suit(&s))
        })
    }

    /// ```
    /// use brydz_core::contract::{TrickGen, TrickSolver, TrumpTrickSolver};
    /// use brydz_core::player::side::Side::*;
    /// use karty::cards::*;
    /// use karty::suits::Suit::Hearts;
    /// let mut trick1 = TrickGen::new(North);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, ACE_CLUBS).unwrap();
    /// trick1.insert_card(South, KING_HEARTS).unwrap();
    /// assert!(TrumpTrickSolver::new(Hearts).does_beat_leader(&trick1, &ACE_HEARTS));
    /// assert!(!TrumpTrickSolver::new(Hearts).does_beat_leader(&trick1, &JACK_HEARTS));
    /// assert!(!TrumpTrickSolver::new(Hearts).does_beat_leader(&trick1, &ACE_SPADES));

    fn does_beat_leader(&self, trick: &TrickGen<Self::CardType>, card: &Self::CardType) -> bool {
        match trick.leader_in_suit_with_card(&self.trump_suit){
            None => card.suit() == self.trump_suit
                || match trick.called_suit()
                .and_then(|s| trick.leader_in_suit_with_card(&s)){
                    None => true,
                    Some((_s,c)) => c.suit() == card.suit() && card.figure() > c.figure()

            },
            Some((_,c)) => (card.suit() == c.suit()) && card.figure()> c.figure()
        }

    }
}

#[derive(Debug,  Clone, PartialEq, Eq)]
pub struct NoTrumpTrickSolver<Crd: Card2SymTrait>{
    _phantom: PhantomData<Crd>,
}
impl <Crd: Card2SymTrait> NoTrumpTrickSolver<Crd>{
    pub fn new() -> Self{
        Self{_phantom: PhantomData::default()}
    }
}

impl <Crd: Card2SymTrait> TrickSolver for NoTrumpTrickSolver<Crd> {
    type CardType = Crd;

    /// Tries to pick a winner of a trick
    /// ```
    ///
    /// use brydz_core::contract::{NoTrumpTrickSolver, TrickGen, TrickSolver};
    /// use brydz_core::error::TrickErrorGen::MissingCard;
    /// use brydz_core::player::side::Side::{*};
    /// use karty::cards::{*};
    /// let mut trick1 = TrickGen::new(North);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, TWO_CLUBS).unwrap();
    /// trick1.insert_card(South, ACE_SPADES).unwrap();
    /// assert_eq!(NoTrumpTrickSolver::new().taker(&trick1), Err(MissingCard(West)));
    /// trick1.insert_card(West, TEN_SPADES).unwrap();
    /// assert_eq!(NoTrumpTrickSolver::new().taker(&trick1), Ok(North));
    /// trick1.undo().unwrap();
    /// trick1.insert_card(West, KING_HEARTS).unwrap();
    /// assert_eq!(NoTrumpTrickSolver::new().taker(&trick1), Ok(West));
    fn taker(&self, trick: &TrickGen<Self::CardType>) -> Result<Side, TrickErrorGen<Self::CardType>> {
        match trick.is_complete(){
            false => Err(MissingCard(trick.first_player_side().next_i(trick.count_cards()))),
            true => {
                trick.leader_in_suit(&trick.called_suit()
                    .unwrap_or_else(|| panic!("It is a bug: No leader selected in called suit {trick:?}")))
                    .ok_or(TrickErrorGen::TrickEmpty)

            }
        }
    }
    /// ```
    /// use brydz_core::contract::{NoTrumpTrickSolver, TrickGen, TrickSolver};
    /// use brydz_core::player::side::Side::*;
    /// use karty::cards::*;
    /// use karty::suits::Suit::*;
    /// let mut trick1 = TrickGen::new(North);
    /// assert_eq!(NoTrumpTrickSolver::new().leader(&trick1), None);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, ACE_CLUBS).unwrap();
    /// trick1.insert_card(South, KING_HEARTS).unwrap();
    /// trick1.insert_card(West, TEN_SPADES).unwrap();
    /// assert_eq!(NoTrumpTrickSolver::new().leader(&trick1), Some(South));
    /// ```
    fn leader(&self, trick: &TrickGen<Self::CardType>) -> Option<Side> {
        trick.called_suit().and_then(|s|trick.leader_in_suit(&s))
    }

    /// ```
    /// use brydz_core::contract::{NoTrumpTrickSolver, TrickGen, TrickSolver};
    /// use brydz_core::player::side::Side::*;
    /// use karty::cards::*;
    /// use karty::suits::Suit::Hearts;
    /// let mut trick1 = TrickGen::new(North);
    /// trick1.insert_card(North, QUEEN_HEARTS).unwrap();
    /// trick1.insert_card(East, ACE_CLUBS).unwrap();
    /// trick1.insert_card(South, KING_HEARTS).unwrap();
    /// assert!(NoTrumpTrickSolver::new().does_beat_leader(&trick1, &ACE_HEARTS));
    /// assert!(!NoTrumpTrickSolver::new().does_beat_leader(&trick1, &JACK_HEARTS));
    /// assert!(!NoTrumpTrickSolver::new().does_beat_leader(&trick1, &ACE_SPADES));
    fn does_beat_leader(&self, trick: &TrickGen<Self::CardType>, card: &Self::CardType) -> bool {

        match trick.leader_in_called_suit_with_card(){
            None => true,
            Some((_s,c)) => c.suit() == card.suit() && c.figure() < card.figure()
        }
    }
}