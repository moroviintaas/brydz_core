use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::ops::Index;
use karty::cards::{Card2SymTrait, Card};
use karty::register::{Register, CardRegister};
use crate::cards::trump::Trump;
use crate::contract::collision::{SuitExhaust, TrickCollision};
use crate::contract::spec::ContractSpec;
use crate::contract::maintainer::ContractMechanics;
use crate::contract::TrickGen;
use crate::error::ContractErrorGen;
use crate::error::ContractErrorGen::IndexedOverCurrentTrick;
use crate::error::TrickErrorGen::MissingCard;
use crate::meta::{MAX_INDEX_IN_DEAL, QUARTER_SIZE};
use crate::player::axis::Axis;
use crate::player::side::Side;

#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct ContractGen<Crd: Card2SymTrait, Um: Register<Crd>, Se:Register<(Side, Crd::Suit)>>{
    contract_spec: ContractSpec<Crd::Suit>,
    tricks: [TrickGen<Crd>; QUARTER_SIZE],
    completed_tricks_number: usize,
    exhaust_table: Se,
    current_trick: TrickGen<Crd>,
    used_cards_memory: Um

}

impl<Crd: Card2SymTrait,
    Um: Register<Crd>,
    Se:Register<(Side, Crd::Suit)>> ContractMechanics for ContractGen<Crd, Um, Se>{

    type Card = Crd;

    fn current_trick(&self) -> &TrickGen<Self::Card>{
        &self.current_trick
    }
    fn contract_spec(&self) -> &ContractSpec<Crd::Suit>{
        &self.contract_spec
    }
    fn count_completed_tricks(&self) -> usize{
        self.completed_tricks_number
    }
    /// Inserts card to current trick in contract. If trick is closed (contains a card from each side (4)) it is closed and added to array of completed tricks.
    /// # Returns:
    /// `Ok(())` if card has been successfully added
    /// `Err(DealError)` Hopefully an error describing problem
    ///
    /// # Examples:
    /// ```
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::bidding::Doubling;
    /// use brydz_core::contract::{ContractSpec};
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::contract::Contract;
    /// use brydz_core::error::ContractErrorGen;
    /// use brydz_core::player::side::Side;
    /// use std::str::FromStr;
    /// use brydz_core::contract::ContractMechanics;
    /// use brydz_core::player::axis::Axis;
    /// use brydz_core::error::TrickErrorGen;
    /// use brydz_core::contract::collision::{SuitExhaust};
    /// use karty::figures::Figure;
    /// use karty::suits::Suit;
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    /// let mut contract = Contract::new(
    ///     ContractSpec::new(Side::West, Bid::init(Trump::Colored(Suit::Hearts), 1).unwrap(),));
    /// contract.insert_card(Side::North, KING_HEARTS).unwrap();
    /// contract.insert_card(Side::East, ACE_HEARTS).unwrap();
    /// contract.insert_card(Side::South, TWO_CLUBS).unwrap();
    /// assert_eq!(contract.count_completed_tricks(), 0);
    /// let r = contract.insert_card(Side::West, SEVEN_HEARTS);
    /// assert_eq!(r.unwrap(), Side::East);
    /// assert_eq!(contract.count_completed_tricks(), 1);
    /// assert_eq!(contract.side_winning_trick(0).unwrap(), Side::East);
    /// let r = contract.insert_card(Side::East, TEN_HEARTS);
    /// assert_eq!(r.unwrap(), Side::South);
    /// let r = contract.insert_card(Side::South, JACK_HEARTS);
    /// assert_eq!(r, Err(ContractErrorGen::TrickError(TrickErrorGen::UsedPreviouslyExhaustedSuit(Suit::Hearts))));
    /// contract.insert_card(Side::South, TWO_CLUBS).unwrap();
    /// contract.insert_card(Side::West, SIX_HEARTS).unwrap();
    /// let r = contract.insert_card(Side::North, THREE_HEARTS);
    ///
    /// assert_eq!(r, Err(ContractErrorGen::DuplicateCard(TWO_CLUBS)));
    ///
    /// ```
    fn insert_card(&mut self, side: Side, card: Crd) -> Result<Side, ContractErrorGen<Crd>>{
        if self.completed_tricks_number >= QUARTER_SIZE{
            return Err(ContractErrorGen::DealFull);
        }
        match self.current_trick.add_card(side, card, &mut self.exhaust_table){
            Ok(4) => {
                match self.current_trick.taker(self.trump()){
                    Ok(winner) => {
                        match self.complete_current_trick(){
                            Ok(()) => Ok(winner),
                            Err(e) => Err(e)

                        }

                    }
                    Err(e) => Err(ContractErrorGen::TrickError( e))
                }
            },
            Ok(_) => Ok(side.next()),
            Err(e) => Err(ContractErrorGen::TrickError( e))

        }
    }
    fn is_completed(&self) -> bool{
        match self.completed_tricks_number {
            n if n < QUARTER_SIZE => false,
            QUARTER_SIZE => true,
            //Infallible, I guess
            _ => panic!("Number of tricks in contract should never ever exceed {}.", QUARTER_SIZE)
        }
    }

    fn completed_tricks(&self) -> Vec<TrickGen<Crd>> {
        let mut r = Vec::new();
        for i in 0..self.completed_tricks_number{
            r.push(self.tricks[i].to_owned());
        }
        r
    }
    /// Counts tricks taken by `Side` (one player)
    /// # Examples:
    /// ```
    /// use brydz_core::player::side::Side::*;
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::contract::{ContractMechanics,Contract};
    /// use std::str::FromStr;
    /// use brydz_core::contract::{ContractSpec};
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::bidding::Doubling;
    /// use brydz_core::contract::collision::{SuitExhaust};
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    ///
    /// let mut deal = Contract::new(ContractSpec::new(West, Bid::init(Trump::Colored(Diamonds), 1).unwrap(),));
    ///
    /// deal.insert_card(North, JACK_SPADES).unwrap();
    /// deal.insert_card(East, TEN_SPADES).unwrap();
    /// deal.insert_card(South, FOUR_SPADES).unwrap();
    /// deal.insert_card(West, FIVE_DIAMONDS).unwrap(); //winner
    ///
    /// deal.insert_card(West, EIGHT_HEARTS).unwrap();
    /// deal.insert_card(North, JACK_DIAMONDS).unwrap(); //winner
    /// deal.insert_card(East, KING_HEARTS).unwrap();
    /// deal.insert_card(South, NINE_HEARTS).unwrap();
    ///
    /// deal.insert_card(North, ACE_CLUBS).unwrap(); //winner
    /// deal.insert_card(East, QUEEN_SPADES).unwrap();
    /// deal.insert_card(South, SEVEN_HEARTS).unwrap();
    /// deal.insert_card(West, FOUR_CLUBS).unwrap();
    /// assert_eq!(deal.total_tricks_taken_side(North), 2);
    /// assert_eq!(deal.total_tricks_taken_side(West), 1);
    /// assert_eq!(deal.total_tricks_taken_side(South), 0);
    /// assert_eq!(deal.total_tricks_taken_side(East), 0);
    /// ```
    fn total_tricks_taken_side(&self, side: Side) -> usize{
        self.tricks[0..self.completed_tricks_number].iter().filter(|t| t.taker(self.contract_spec.bid().trump()).unwrap() == side).count()
    }
    /// Counts tricks taken by `Side` (one player)
    /// # Examples:
    /// ```
    /// use brydz_core::player::side::Side::*;
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::contract::{ContractMechanics, Contract};
    /// use std::str::FromStr;
    /// use brydz_core::player::axis::Axis;
    /// use brydz_core::bidding::Doubling;
    /// use brydz_core::contract::{ContractSpec};
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::contract::collision::{SuitExhaust};
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    /// let mut deal = Contract::new(ContractSpec::new(West, Bid::init(Trump::Colored(Diamonds), 1).unwrap(),));
    /// deal.insert_card(North, JACK_SPADES).unwrap();
    /// deal.insert_card(East, TEN_SPADES).unwrap();
    /// deal.insert_card(South, FOUR_SPADES).unwrap();
    /// deal.insert_card(West, FIVE_DIAMONDS).unwrap(); //winner
    ///
    /// deal.insert_card(West, EIGHT_HEARTS).unwrap();
    /// deal.insert_card(North, JACK_DIAMONDS).unwrap(); //winner
    /// deal.insert_card(East, KING_HEARTS).unwrap();
    /// deal.insert_card(South, NINE_HEARTS).unwrap();
    ///
    /// deal.insert_card(North, ACE_CLUBS).unwrap(); //winner
    /// deal.insert_card(East, QUEEN_SPADES).unwrap();
    /// deal.insert_card(South, SEVEN_HEARTS).unwrap();
    /// deal.insert_card(West, FOUR_CLUBS).unwrap();
    /// assert_eq!(deal.total_tricks_taken_axis(Axis::NorthSouth), 2);
    /// assert_eq!(deal.total_tricks_taken_axis(Axis::EastWest), 1);
    /// ```
    fn total_tricks_taken_axis(&self, axis: Axis) -> usize{
        self.tricks[0..self.completed_tricks_number].iter().filter(|t| t.taker(self.contract_spec.bid().trump()).unwrap().axis() == axis).count()
    }

}

impl<Card: Card2SymTrait, Um: Register<Card>, Se: Register<(Side, Card::Suit)>> ContractGen<Card, Um, Se>{
    pub fn new(contract: ContractSpec<Card::Suit>) -> Self{
        let first_player = contract.declarer().next();
        let mut tricks = <[TrickGen::<Card>; QUARTER_SIZE]>::default();
        tricks[0] = TrickGen::new(first_player);
        Self{
            contract_spec: contract, tricks, completed_tricks_number: 0,
            exhaust_table: Se::default(), current_trick: TrickGen::new(first_player), used_cards_memory: Um::default()}
    }


    fn complete_current_trick(&mut self) -> Result<(), ContractErrorGen<Card>>{
        match self.completed_tricks_number {
            n@0..=MAX_INDEX_IN_DEAL => match self.current_trick.missing_card(){
                Some(s) => Err(ContractErrorGen::TrickError( MissingCard(s))),
                None => {
                    if let Some(c) = self.used_cards_memory.trick_collision(&self.current_trick){
                        return Err(ContractErrorGen::DuplicateCard(c));
                    }

                    let next_player = self.current_trick.taker(self.trump()).unwrap();

                    self.used_cards_memory.mark_cards_of_trick(&self.current_trick);
                    self.tricks[n] = mem::replace(&mut self.current_trick, TrickGen::new(next_player));

                    //self.current_trick = Trick::new(next_player);
                    self.completed_tricks_number = n+1;
                    Ok(())
                }

            }
            //full if full >= QUARTER_SIZE => Err(DealError::DealFull),
            _ => Err(ContractErrorGen::DealFull),
        }
    }


    pub fn trump(&self) -> &Trump<Card::Suit>{
        self.contract_spec.bid().trump()
    }
    pub fn last_completed_trick(&self) -> Option<&TrickGen<Card>>{
        match self.completed_tricks_number {
            0 => None,
            i @1..=QUARTER_SIZE => Some(&self[i-1]),
            _ => panic!("Deal::Last_trick: contract overflow shouldn't happen")

        }
    }

    pub fn init_new_trick(&self) -> Option<TrickGen<Card>>{
        //println!("{:?}", self.trump());
        match self.last_completed_trick(){
            None => Some(TrickGen::new(self.contract_spec.declarer().prev())),

            Some(t) => t.prepare_new(self.trump().to_owned())
        }

    }


    /// Based on index of trick returns the side who won the trick.
    /// # Examples:
    /// ```
    /// use brydz_core::player::side::Side::*;
    /// use brydz_core::contract::TrickGen;
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::cards::deck::Deck;
    /// use brydz_core::player::side::SIDES;
    /// use brydz_core::contract::{ContractMechanics,Contract};
    /// use std::str::FromStr;
    /// use brydz_core::bidding::Doubling;
    /// use brydz_core::contract::{ContractSpec};
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::contract::collision::{SuitExhaust};
    /// use karty::figures::Figure;
    /// use karty::suits::{Suit, Suit::*};
    /// use karty::register::CardRegister;
    /// use karty::cards::*;
    /// let deck = Deck::new_sorted_by_figures();
    /// let mut deal_1 = Contract::new(ContractSpec::new_d(North, Bid::init(Trump::Colored(Diamonds), 1).unwrap(), Doubling::None));
    ///
    /// deal_1.insert_card(East, KING_SPADES).unwrap();
    /// deal_1.insert_card(South, QUEEN_SPADES).unwrap();
    /// deal_1.insert_card(West, JACK_SPADES).unwrap();
    /// deal_1.insert_card(North, ACE_SPADES).unwrap();
    /// assert_eq!(deal_1.side_winning_trick(0), Ok(North));
    ///
    /// let mut deal_2 = Contract::new(ContractSpec::new_d(West, Bid::init(Trump::NoTrump, 1u8).unwrap(), Doubling::None));
    ///
    /// deal_2.insert_card(North, TWO_DIAMONDS).unwrap();
    /// deal_2.insert_card(East, ACE_CLUBS).unwrap();
    /// deal_2.insert_card(South, QUEEN_CLUBS).unwrap();
    /// deal_2.insert_card(West, THREE_DIAMONDS).unwrap();
    /// assert_eq!(deal_2.side_winning_trick(0), Ok(West));
    /// deal_2.insert_card(West, FOUR_DIAMONDS).unwrap();
    /// deal_2.insert_card(North, JACK_DIAMONDS).unwrap();
    /// deal_2.insert_card(East, KING_CLUBS).unwrap();
    /// deal_2.insert_card(South, NINE_SPADES).unwrap();
    /// //deal_2.insert_trick(trick_2_2).unwrap();
    /// assert_eq!(deal_2.side_winning_trick(1), Ok(North));
    /// ```
    pub fn side_winning_trick(&self, index: usize) -> Result<Side, ContractErrorGen<Card>>{
        match index < self.completed_tricks_number {
            true => self[index].taker(self.contract_spec.bid().trump())
                .map_err(|trick_err| ContractErrorGen::TrickError(trick_err)),
            false => Err(IndexedOverCurrentTrick(self.completed_tricks_number))
        }
    }

    pub fn used_cards(&self) -> &Um{
        &self.used_cards_memory
    }





}

impl<Card: Card2SymTrait,
    Um: Register<Card> + Debug,
    Se: Register<(Side,Card::Suit)> + std::fmt::Debug>  Display for ContractGen<Card, Um, Se>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}



impl<Card: Card2SymTrait, Um: Register<Card>, Se: Register<(Side, Card::Suit)>> Index<usize> for ContractGen<Card, Um, Se>{
    type Output = TrickGen<Card>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tricks[index]
    }
}

pub type Contract = ContractGen<Card, CardRegister, SuitExhaust>;

#[cfg(test)]
mod tests{
    use karty::cards::{*};
    use karty::register::CardRegister;
    use karty::suits::Suit::Diamonds;
    use crate::cards::trump::Trump;
    use crate::contract::spec::{ContractSpec};
    use crate::bidding::Bid;
    use crate::contract::collision::SuitExhaust;
    use crate::contract::maintainer::{ContractMechanics};
    use crate::cards::deck::{Deck};
    use crate::contract::ContractGen;
    use crate::error::ContractErrorGen;
    use crate::error::ContractErrorGen::DealFull;
    use crate::meta::QUARTER_SIZE;
    use crate::player::side::Side;
    use crate::player::side::Side::{East, North, South, West};


    #[test]
    fn deal_duplicate_card(){
        let mut deal = ContractGen::<Card, CardRegister, SuitExhaust>::new(ContractSpec::new(West, Bid::init(Trump::NoTrump, 1).unwrap(), ));
        //let deck = Deck::new_sorted_by_suits();



        deal.insert_card(Side::North, ACE_SPADES).unwrap();
        deal.insert_card(Side::East, QUEEN_SPADES).unwrap();
        deal.insert_card(Side::South, KING_SPADES).unwrap();
        deal.insert_card(Side::West, JACK_SPADES).unwrap();

        deal.insert_card(Side::North, ACE_SPADES).unwrap();
        deal.insert_card(Side::East, KING_CLUBS).unwrap();
        deal.insert_card(Side::South, JACK_CLUBS.clone()).unwrap();

        let r = deal.insert_card(Side::West, TEN_HEARTS.clone());


        assert_eq!(r, Err(ContractErrorGen::DuplicateCard(ACE_SPADES)));

    }


    #[test]
    fn deal_overflow_tricks(){
        let num_of_sides = 4usize;
        let deck = Deck::new_sorted_by_suits();
        //let mut contract = Deal::new(South, Trump::NoTrump);
        let mut deal = ContractGen::<Card, CardRegister, SuitExhaust>::new(
            ContractSpec::new(
                West, Bid::init(Trump::NoTrump, 1).unwrap(), ));
        for i in 0..QUARTER_SIZE{

            deal.insert_card(Side::North,deck[num_of_sides*i].clone()).unwrap();
            deal.insert_card(Side::East,deck[num_of_sides*i + 1].clone()).unwrap();
            deal.insert_card(Side::South,deck[num_of_sides*i + 2].clone()).unwrap();
            deal.insert_card(Side::West,deck[num_of_sides*i +3].clone()).unwrap();

        }

        let r = deal.insert_card(Side::North, deck[0]);
        assert_eq!(r, Err(DealFull));



    }

    #[test]
    fn calculate_score_1(){
        let mut deal = ContractGen::<Card, CardRegister, SuitExhaust>::new(ContractSpec::new(
            East,
            Bid::init(Trump::Colored(Diamonds), 3).unwrap(),
        ));
        deal.insert_card(South, ACE_SPADES).unwrap();
        deal.insert_card(West, THREE_SPADES).unwrap();
        deal.insert_card(North, FOUR_SPADES).unwrap();
        deal.insert_card(East, SIX_SPADES).unwrap();
        assert_eq!(deal.count_completed_tricks(), 1);

        deal.insert_card(South, TWO_SPADES).unwrap();
        deal.insert_card(West, THREE_DIAMONDS).unwrap();
        deal.insert_card(North, EIGHT_SPADES).unwrap();
        deal.insert_card(East, TEN_SPADES).unwrap();
        assert_eq!(deal.count_completed_tricks(), 2);

        deal.insert_card(West, FOUR_DIAMONDS).unwrap();
        deal.insert_card(North, JACK_DIAMONDS).unwrap();
        deal.insert_card(East, QUEEN_DIAMONDS).unwrap();
        deal.insert_card(South, TEN_DIAMONDS).unwrap();

        deal.insert_card(East, KING_DIAMONDS).unwrap();
        deal.insert_card(South, FIVE_SPADES).unwrap();
        deal.insert_card(West, FIVE_DIAMONDS).unwrap();
        deal.insert_card(North, NINE_DIAMONDS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 4);

        deal.insert_card(East, FOUR_HEARTS).unwrap();
        deal.insert_card(South, FIVE_HEARTS).unwrap();
        deal.insert_card(West, KING_HEARTS).unwrap();
        deal.insert_card(North, ACE_HEARTS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 5);

        deal.insert_card(North, FIVE_CLUBS).unwrap();
        deal.insert_card(East, ACE_CLUBS).unwrap();
        deal.insert_card(South, FOUR_CLUBS).unwrap();
        deal.insert_card(West, TWO_CLUBS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 6);

        deal.insert_card(East, QUEEN_HEARTS).unwrap();
        deal.insert_card(South, EIGHT_HEARTS).unwrap();
        deal.insert_card(West, THREE_HEARTS).unwrap();
        deal.insert_card(North, TWO_HEARTS).unwrap();

        deal.insert_card(East, QUEEN_SPADES).unwrap();
        deal.insert_card(South, SEVEN_SPADES).unwrap();
        deal.insert_card(West, SIX_DIAMONDS).unwrap();
        deal.insert_card(North, NINE_SPADES).unwrap();
        assert_eq!(deal.count_completed_tricks(), 8);

        deal.insert_card(West, KING_CLUBS).unwrap();
        deal.insert_card(North, NINE_CLUBS).unwrap();
        deal.insert_card(East, TEN_HEARTS).unwrap();
        deal.insert_card(South, SIX_CLUBS).unwrap();

        deal.insert_card(West, THREE_CLUBS).unwrap();
        deal.insert_card(North, QUEEN_CLUBS).unwrap();
        deal.insert_card(East, TWO_DIAMONDS).unwrap();
        deal.insert_card(South, SEVEN_CLUBS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 10);

        deal.insert_card(East, SEVEN_DIAMONDS).unwrap();
        deal.insert_card(South, EIGHT_CLUBS).unwrap();
        deal.insert_card(West, SIX_HEARTS).unwrap();
        deal.insert_card(North, SEVEN_HEARTS).unwrap();
        assert_eq!(deal.count_completed_tricks(), 11);

        deal.insert_card(East, EIGHT_DIAMONDS).unwrap();
        deal.insert_card(South, JACK_CLUBS).unwrap();
        deal.insert_card(West, NINE_HEARTS).unwrap();
        deal.insert_card(North, JACK_SPADES).unwrap();
        assert_eq!(deal.count_completed_tricks(), 12);

        deal.insert_card(East, ACE_DIAMONDS).unwrap();
        deal.insert_card(South, JACK_HEARTS).unwrap();
        deal.insert_card(West, TEN_CLUBS).unwrap();
        deal.insert_card(North, KING_SPADES).unwrap();


        //assert_eq!(contract.completed_tricks(), 13);
        assert_eq!(deal.total_tricks_taken_side(East), 8);
        assert_eq!(deal.total_tricks_taken_side(South), 1);
        assert_eq!(deal.total_tricks_taken_side(West), 3);
        assert_eq!(deal.total_tricks_taken_side(North), 1);


    }





}