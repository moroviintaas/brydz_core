use std::fmt::{Display, Formatter};
use std::mem;
use std::ops::Index;
use karty::cards::Card;
use karty::figures::Figure;
use karty::card_register::register::{Register};
use karty::suits::Suit;
use crate::play::trick::{Trick, TrickError};
use crate::player::side::Side;
use crate::play::trump::Trump;
use crate::player::axis::Axis;
use crate::auction::contract::Contract;
use crate::play::deal::DealError::IndexedOverCurrentTrick;
use crate::play::deck::{MAX_INDEX_IN_DEAL, QUARTER_SIZE};
use crate::play::card_trackers::{SuitExhaustRegister, TrickCollision};
use crate::score::score_table::ScoreTable;
use crate::play::trick::TrickError::MissingCard;




#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealError<F: Figure, S: Suit>{
    DealFull,
    DuplicateCard(Card<F, S>),
    TrickError(TrickError<F, S>),
    IndexedOverCurrentTrick(usize)

}
impl<F: Figure, S: Suit>Display for DealError<F, S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}



#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct Deal<F: Figure, S: Suit, Um: Register<Card<F,S>>, Se: SuitExhaustRegister<S>>{
    contract: Contract<S>,
    tricks: [Trick<F, S>; QUARTER_SIZE],
    completed_tricks_number: usize,
    exhaust_table: Se,
    current_trick: Trick<F, S>,
    used_cards_memory: Um

}
impl<F: Figure, S: Suit, Um: Register<Card<F,S>>, Se: SuitExhaustRegister<S>> Deal<F, S, Um, Se>{
    pub fn new(contract: Contract<S>) -> Self{
        let first_player = contract.declarer().next();
        let mut tricks = <[Trick::<F,S>; QUARTER_SIZE]>::default();
        tricks[0] = Trick::new(first_player);
        Self{contract, tricks, completed_tricks_number: 0,
            exhaust_table: Se::default(), current_trick: Trick::new(first_player), used_cards_memory: Um::default()}
    }

    pub fn current_trick(&self) -> &Trick<F, S>{
        &self.current_trick
    }

    fn complete_current_trick(&mut self) -> Result<(), DealError<F, S>>{
        match self.completed_tricks_number {
            n@0..=MAX_INDEX_IN_DEAL => match self.current_trick.missing_card(){
                Some(s) => Err(DealError::TrickError( MissingCard(s))),
                None => {
                    if let Some(c) = self.used_cards_memory.trick_collision(&self.current_trick){
                        return Err(DealError::DuplicateCard(c));
                    }

                    let next_player = self.current_trick.taker(self.trump()).unwrap();

                    self.used_cards_memory.mark_cards_of_trick(&self.current_trick);
                    self.tricks[n] = mem::replace( &mut self.current_trick, Trick::new(next_player));

                    //self.current_trick = Trick::new(next_player);
                    self.completed_tricks_number = n+1;
                    Ok(())
                }

            }
            //full if full >= QUARTER_SIZE => Err(DealError::DealFull),
            _ => Err(DealError::DealFull),
        }
    }

    pub fn completed_tricks(&self) -> usize{
        self.completed_tricks_number
    }

    /// Inserts card to current trick in deal. If trick is closed (contains a card from each side (4)) it is closed and added to array of completed tricks.
    /// # Returns:
    /// `Ok(())` if card has been successfully added
    /// `Err(DealError)` Hopefully an error describing problem
    ///
    /// # Examples:
    /// ```
    /// use bridge_core::play::trump::Trump;
    /// use bridge_core::auction::call::Doubling;
    /// use bridge_core::auction::contract::{Contract};
    /// use bridge_core::auction::bid::Bid;
    /// use bridge_core::play::deal::{Deal, DealError};
    /// use bridge_core::player::side::Side;
    /// use std::str::FromStr;
    /// use bridge_core::player::axis::Axis;
    /// use bridge_core::play::trick::TrickError;
    /// use bridge_core::play::card_trackers::{SuitExhaustStd};
    /// use karty::figures::FigureStd;
    /// use karty::suits::SuitStd;
    /// use karty::card_register::standard_register::CardUsageRegStd;
    /// use karty::cards::standard::*;
    /// let mut deal = Deal::<FigureStd, SuitStd, CardUsageRegStd, SuitExhaustStd>::new(
    ///     Contract::new(Side::West, Bid::create_bid(Trump::Colored(SuitStd::Hearts), 1).unwrap()));
    /// deal.insert_card(Side::North, KING_HEARTS).unwrap();
    /// deal.insert_card(Side::East, ACE_HEARTS).unwrap();
    /// deal.insert_card(Side::South, TWO_CLUBS).unwrap();
    /// assert_eq!(deal.completed_tricks(), 0);
    /// let r = deal.insert_card(Side::West, SEVEN_HEARTS);
    /// assert_eq!(r.unwrap(), Side::East);
    /// assert_eq!(deal.completed_tricks(), 1);
    /// assert_eq!(deal.side_winning_trick(0).unwrap(), Side::East);
    /// let r = deal.insert_card(Side::East, TEN_HEARTS);
    /// assert_eq!(r.unwrap(), Side::South);
    /// let r = deal.insert_card(Side::South, JACK_HEARTS);
    /// assert_eq!(r, Err(DealError::TrickError(TrickError::UsedPreviouslyExhaustedSuit(SuitStd::Hearts))));
    /// deal.insert_card(Side::South, TWO_CLUBS).unwrap();
    /// deal.insert_card(Side::West, SIX_HEARTS).unwrap();
    /// let r = deal.insert_card(Side::North, THREE_HEARTS);
    ///
    /// assert_eq!(r, Err(DealError::DuplicateCard(TWO_CLUBS)));
    ///
    /// ```
    pub fn insert_card(&mut self, side: Side, card: Card<F, S>) -> Result<Side, DealError<F, S>>{
        if self.completed_tricks_number >= QUARTER_SIZE{
            return Err(DealError::DealFull);
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
                    Err(e) => Err(DealError::TrickError( e))
                }
            },
            Ok(_) => Ok(side.next()),
            Err(e) => Err(DealError::TrickError( e))

        }
    }

    pub fn trump(&self) -> &Trump<S>{
        self.contract.bid().trump()
    }
    pub fn last_completed_trick(&self) -> Option<&Trick<F, S>>{
        match self.completed_tricks_number {
            0 => None,
            i @1..=QUARTER_SIZE => Some(&self[i-1]),
            _ => panic!("Deal::Last_trick: deal overflow shouldn't happen")

        }
    }

    pub fn init_new_trick(&self) -> Option<Trick<F, S>>{
        //println!("{:?}", self.trump());
        match self.last_completed_trick(){
            None => Some(Trick::new(self.contract.declarer().prev())),

            Some(t) => t.prepare_new(self.trump().to_owned())
        }

    }


    /// Based on index of trick returns the side who won the trick.
    /// # Examples: 
    /// ```
    /// use bridge_core::player::side::Side::*;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::play::trump::Trump;
    /// use bridge_core::play::deck::Deck;
    /// use bridge_core::player::side::SIDES;
    /// use bridge_core::play::deal::Deal;
    /// use std::str::FromStr;
    /// use bridge_core::auction::call::Doubling;
    /// use bridge_core::auction::contract::{Contract};
    /// use bridge_core::auction::bid::Bid;
    /// use bridge_core::play::card_trackers::{SuitExhaustStd};
    /// use karty::figures::FigureStd;
    /// use karty::suits::{SuitStd, SuitStd::*};
    /// use karty::card_register::standard_register::CardUsageRegStd;
    /// use karty::cards::standard::*;
    /// let deck = Deck::new_sorted_by_figures();
    /// let mut deal_1 = Deal::<FigureStd, SuitStd, CardUsageRegStd, SuitExhaustStd>::new(Contract::new_d(North, Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap(), Doubling::None));
    ///
    /// deal_1.insert_card(East, KING_SPADES).unwrap();
    /// deal_1.insert_card(South, QUEEN_SPADES).unwrap();
    /// deal_1.insert_card(West, JACK_SPADES).unwrap();
    /// deal_1.insert_card(North, ACE_SPADES).unwrap();
    /// assert_eq!(deal_1.side_winning_trick(0), Ok(North));
    ///
    /// let mut deal_2 = Deal::<FigureStd, SuitStd, CardUsageRegStd, SuitExhaustStd>::new(Contract::new_d(West, Bid::create_bid(Trump::NoTrump, 1).unwrap(), Doubling::None));
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
    pub fn side_winning_trick(&self, index: usize) -> Result<Side, DealError<F, S>>{
        match index < self.completed_tricks_number {
            true => self[index].taker(self.contract.bid().trump())
                .map_err(|trick_err| DealError::TrickError(trick_err)),
            false => Err(IndexedOverCurrentTrick(self.completed_tricks_number))
        }
    }
    /// Counts tricks taken by `Side` (one player)
    /// # Examples:
    /// ```
    /// use bridge_core::player::side::Side::*;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::play::trump::Trump;
    /// use bridge_core::play::deal::Deal;
    /// use std::str::FromStr;
    /// use bridge_core::auction::contract::{Contract};
    /// use bridge_core::auction::bid::Bid;
    /// use bridge_core::auction::call::Doubling;
    /// use bridge_core::play::card_trackers::{SuitExhaustStd};
    /// use karty::figures::FigureStd;
    /// use karty::suits::{SuitStd, SuitStd::*};
    /// use karty::card_register::standard_register::CardUsageRegStd;
    /// use karty::cards::standard::*;
    ///
    /// let mut deal = Deal::<FigureStd, SuitStd, CardUsageRegStd, SuitExhaustStd>::new(Contract::new(West, Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap()));
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
    /// assert_eq!(deal.tricks_taken(North), 2);
    /// assert_eq!(deal.tricks_taken(West), 1);
    /// assert_eq!(deal.tricks_taken(South), 0);
    /// assert_eq!(deal.tricks_taken(East), 0);
    /// ```
    pub fn tricks_taken(&self, side: Side) -> usize{
        self.tricks[0..self.completed_tricks_number].iter().filter(|t| t.taker(self.contract.bid().trump()).unwrap() == side).count()
    }
    /// Counts tricks taken by `Side` (one player)
    /// # Examples:
    /// ```
    /// use bridge_core::player::side::Side::*;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::play::trump::Trump;
    /// use bridge_core::play::deal::Deal;
    /// use std::str::FromStr;
    /// use bridge_core::player::axis::Axis;
    /// use bridge_core::auction::call::Doubling;
    /// use bridge_core::auction::contract::{Contract};
    /// use bridge_core::auction::bid::Bid;
    /// use bridge_core::play::card_trackers::{SuitExhaustStd};
    /// use karty::figures::FigureStd;
    /// use karty::suits::{SuitStd, SuitStd::*};
    /// use karty::card_register::standard_register::CardUsageRegStd;
    /// use karty::cards::standard::*;
    /// let mut deal = Deal::<FigureStd, SuitStd, CardUsageRegStd, SuitExhaustStd>::new(Contract::new(West, Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap()));
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
    /// assert_eq!(deal.tricks_taken_axis(Axis::NorthSouth), 2);
    /// assert_eq!(deal.tricks_taken_axis(Axis::EastWest), 1);
    /// ```
    pub fn tricks_taken_axis(&self, axis: Axis) -> usize{
        self.tricks[0..self.completed_tricks_number].iter().filter(|t| t.taker(self.contract.bid().trump()).unwrap().axis() == axis).count()
    }

    pub fn is_completed(&self) -> bool{
        match self.completed_tricks_number {
            n if n < QUARTER_SIZE => false,
            QUARTER_SIZE => true,
            //Infallible, I guess
            _ => panic!("Number of tricks in deal should never ever exceed {}.", QUARTER_SIZE)
        }
    }

    pub fn score(&self) -> ScoreTable {
         ScoreTable::default()

        
    }

}

impl<F: Figure, S: Suit, Um: Register<Card<F,S>>, Se: SuitExhaustRegister<S>>Display for Deal<F, S, Um, Se>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ClosedDealRubber<F: Figure, S: Suit, Um: Register<Card<F,S>>, Se: SuitExhaustRegister<S>> {
    contract: Deal<F, S, Um, Se>,
    score: ScoreTable

}
/*
impl ClosedDealRubber{

}*/

impl<F: Figure, S: Suit, Um: Register<Card<F,S>>, Se: SuitExhaustRegister<S>> Index<usize> for Deal<F, S, Um, Se>{
    type Output = Trick<F, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tricks[index]
    }
}

#[cfg(test)]
mod tests{
    use karty::cards::standard::{*};
    use karty::figures::FigureStd;
    use karty::card_register::standard_register::CardUsageRegStd;
    use karty::suits::SuitStd;
    use karty::suits::SuitStd::Diamonds;
    use crate::play::trump::Trump;
    use crate::auction::contract::{Contract};
    use crate::auction::bid::Bid;
    use crate::play::card_trackers::SuitExhaustStd;
    use crate::play::deal::{Deal, DealError};
    use crate::play::deal::DealError::DealFull;
    use crate::play::deck::{Deck, QUARTER_SIZE};
    use crate::player::side::Side;
    use crate::player::side::Side::{East, North, South, West};


    #[test]
    fn deal_duplicate_card(){
        let mut deal = Deal::<FigureStd, SuitStd, CardUsageRegStd, SuitExhaustStd>::new(Contract::new(West, Bid::create_bid(Trump::NoTrump, 1).unwrap()));
        //let deck = Deck::new_sorted_by_suits();



        deal.insert_card(Side::North, ACE_SPADES).unwrap();
        deal.insert_card(Side::East, QUEEN_SPADES).unwrap();
        deal.insert_card(Side::South, KING_SPADES).unwrap();
        deal.insert_card(Side::West, JACK_SPADES).unwrap();

        deal.insert_card(Side::North, ACE_SPADES).unwrap();
        deal.insert_card(Side::East, KING_CLUBS).unwrap();
        deal.insert_card(Side::South, JACK_CLUBS.clone()).unwrap();

        let r = deal.insert_card(Side::West, TEN_HEARTS.clone());


        assert_eq!(r, Err(DealError::DuplicateCard(ACE_SPADES)));

    }


    #[test]
    fn deal_overflow_tricks(){
        let num_of_sides = 4usize;
        let deck = Deck::new_sorted_by_suits();
        //let mut deal = Deal::new(South, Trump::NoTrump);
        let mut deal = Deal::<FigureStd, SuitStd, CardUsageRegStd, SuitExhaustStd>::new(Contract::new(West, Bid::create_bid(Trump::NoTrump, 1).unwrap()));
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
        let mut deal = Deal::<FigureStd, SuitStd, CardUsageRegStd, SuitExhaustStd>::new(Contract::new(
            East,
            Bid::create_bid(Trump::Colored(Diamonds), 3).unwrap()));
        deal.insert_card(South, ACE_SPADES).unwrap();
        deal.insert_card(West, THREE_SPADES).unwrap();
        deal.insert_card(North, FOUR_SPADES).unwrap();
        deal.insert_card(East, SIX_SPADES).unwrap();
        assert_eq!(deal.completed_tricks(), 1);

        deal.insert_card(South, TWO_SPADES).unwrap();
        deal.insert_card(West, THREE_DIAMONDS).unwrap();
        deal.insert_card(North, EIGHT_SPADES).unwrap();
        deal.insert_card(East, TEN_SPADES).unwrap();
        assert_eq!(deal.completed_tricks(), 2);

        deal.insert_card(West, FOUR_DIAMONDS).unwrap();
        deal.insert_card(North, JACK_DIAMONDS).unwrap();
        deal.insert_card(East, QUEEN_DIAMONDS).unwrap();
        deal.insert_card(South, TEN_DIAMONDS).unwrap();

        deal.insert_card(East, KING_DIAMONDS).unwrap();
        deal.insert_card(South, FIVE_SPADES).unwrap();
        deal.insert_card(West, FIVE_DIAMONDS).unwrap();
        deal.insert_card(North, NINE_DIAMONDS).unwrap();
        assert_eq!(deal.completed_tricks(), 4);

        deal.insert_card(East, FOUR_HEARTS).unwrap();
        deal.insert_card(South, FIVE_HEARTS).unwrap();
        deal.insert_card(West, KING_HEARTS).unwrap();
        deal.insert_card(North, ACE_HEARTS).unwrap();
        assert_eq!(deal.completed_tricks(), 5);

        deal.insert_card(North, FIVE_CLUBS).unwrap();
        deal.insert_card(East, ACE_CLUBS).unwrap();
        deal.insert_card(South, FOUR_CLUBS).unwrap();
        deal.insert_card(West, TWO_CLUBS).unwrap();
        assert_eq!(deal.completed_tricks(), 6);

        deal.insert_card(East, QUEEN_HEARTS).unwrap();
        deal.insert_card(South, EIGHT_HEARTS).unwrap();
        deal.insert_card(West, THREE_HEARTS).unwrap();
        deal.insert_card(North, TWO_HEARTS).unwrap();

        deal.insert_card(East, QUEEN_SPADES).unwrap();
        deal.insert_card(South, SEVEN_SPADES).unwrap();
        deal.insert_card(West, SIX_DIAMONDS).unwrap();
        deal.insert_card(North, NINE_SPADES).unwrap();
        assert_eq!(deal.completed_tricks(), 8);

        deal.insert_card(West, KING_CLUBS).unwrap();
        deal.insert_card(North, NINE_CLUBS).unwrap();
        deal.insert_card(East, TEN_HEARTS).unwrap();
        deal.insert_card(South, SIX_CLUBS).unwrap();

        deal.insert_card(West, THREE_CLUBS).unwrap();
        deal.insert_card(North, QUEEN_CLUBS).unwrap();
        deal.insert_card(East, TWO_DIAMONDS).unwrap();
        deal.insert_card(South, SEVEN_CLUBS).unwrap();
        assert_eq!(deal.completed_tricks(), 10);

        deal.insert_card(East, SEVEN_DIAMONDS).unwrap();
        deal.insert_card(South, EIGHT_CLUBS).unwrap();
        deal.insert_card(West, SIX_HEARTS).unwrap();
        deal.insert_card(North, SEVEN_HEARTS).unwrap();
        assert_eq!(deal.completed_tricks(), 11);

        deal.insert_card(East, EIGHT_DIAMONDS).unwrap();
        deal.insert_card(South, JACK_CLUBS).unwrap();
        deal.insert_card(West, NINE_HEARTS).unwrap();
        deal.insert_card(North, JACK_SPADES).unwrap();
        assert_eq!(deal.completed_tricks(), 12);

        deal.insert_card(East, ACE_DIAMONDS).unwrap();
        deal.insert_card(South, JACK_HEARTS).unwrap();
        deal.insert_card(West, TEN_CLUBS).unwrap();
        deal.insert_card(North, KING_SPADES).unwrap();


        //assert_eq!(deal.completed_tricks(), 13);
        assert_eq!(deal.tricks_taken(East), 8);
        assert_eq!(deal.tricks_taken(South), 1);
        assert_eq!(deal.tricks_taken(West), 3);
        assert_eq!(deal.tricks_taken(North), 1);










        //trick.

    }





}