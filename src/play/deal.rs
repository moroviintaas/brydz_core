use std::fmt::{Display, Formatter};
use std::ops::Index;
use crate::play::trick::{Trick, TrickError};
use crate::player::side::Side;
use serde::{Deserialize, Serialize};
use crate::card::Card;
use crate::card::trump::Trump;
use crate::player::axis::Axis;
use crate::auction::contract::Contract;
use crate::play::deal::DealError::IndexedOverCurrentTrick;
use crate::play::deck::{MAX_INDEX_IN_DEAL, QUARTER_SIZE};
use crate::play::exhaust::{SuitExhaust, UsedCards};
use crate::score::score::Score;
use crate::play::trick::TrickError::MissingCard;




#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealError{
    DealFull,
    DuplicateCard(Card),
    TrickError(Trick, TrickError),
    IndexedOverCurrentTrick(usize)

}
impl Display for DealError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}



#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Deal {
    contract: Contract,
    tricks: [Trick; QUARTER_SIZE],
    completed_tricks_number: usize,
    exhaust_table: SuitExhaust,
    current_trick: Trick,
    used_cards_memory: UsedCards

}
impl Deal{
    pub fn new(contract: Contract) -> Self{
        let first_player = contract.owner().next();
        Self{contract, tricks: [Trick::new(first_player);QUARTER_SIZE], completed_tricks_number: 0,
            exhaust_table: SuitExhaust::default(), current_trick: Trick::new(first_player), used_cards_memory: UsedCards::default()}
    }

    pub fn current_trick(&self) -> &Trick{
        &self.current_trick
    }

    fn complete_current_trick(&mut self) -> Result<(), DealError>{
        match self.completed_tricks_number {
            n@0..=MAX_INDEX_IN_DEAL => match self.current_trick.missing_card(){
                Some(s) => Err(DealError::TrickError(self.current_trick, MissingCard(s))),
                None => {
                    if let Some(c) = self.used_cards_memory.trick_collision(self.current_trick){
                        return Err(DealError::DuplicateCard(c));
                    }

                    let next_player = self.current_trick.taker(self.trump()).unwrap();

                    self.used_cards_memory.mark_used_trick(&self.current_trick);
                    self.tricks[n] = self.current_trick;

                    self.current_trick = Trick::new(next_player);
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
    /// use bridge_core::card::Card;
    /// use bridge_core::card::suit::Suit;
    /// use bridge_core::card::trump::Trump;
    /// use bridge_core::auction::call::Doubling;
    /// use bridge_core::auction::contract::{Bid, Contract};
    /// use bridge_core::play::deal::{Deal, DealError};
    /// use bridge_core::player::side::Side;
    /// use bridge_core::card;
    /// use std::str::FromStr;
    /// use bridge_core::player::axis::Axis;
    /// use bridge_core::play::trick::TrickError;
    /// let mut deal = Deal::new(
    ///     Contract::new(Side::West, Bid::create_bid(Trump::Colored(Suit::Hearts), 1).unwrap()));
    /// deal.insert_card(Side::North, card::KING_HEARTS).unwrap();
    /// deal.insert_card(Side::East, card::ACE_HEARTS).unwrap();
    /// deal.insert_card(Side::South, card::TWO_CLUBS).unwrap();
    /// assert_eq!(deal.completed_tricks(), 0);
    /// let r = deal.insert_card(Side::West, card::SEVEN_HEARTS);
    /// assert_eq!(r.unwrap(), Side::East);
    /// assert_eq!(deal.completed_tricks(), 1);
    /// assert_eq!(deal.side_winning_trick(0).unwrap(), Side::East);
    /// let r = deal.insert_card(Side::East, card::TEN_HEARTS);
    /// assert_eq!(r.unwrap(), Side::South);
    /// let r = deal.insert_card(Side::South, card::JACK_HEARTS);
    /// assert_eq!(r, Err(DealError::TrickError(deal.current_trick().to_owned(), TrickError::UsedPreviouslyExhaustedSuit(Suit::Hearts))));
    /// deal.insert_card(Side::South, card::TWO_CLUBS).unwrap();
    /// deal.insert_card(Side::West, card::SIX_HEARTS).unwrap();
    /// let r = deal.insert_card(Side::North, card::THREE_HEARTS);
    ///
    /// assert_eq!(r, Err(DealError::DuplicateCard(card::TWO_CLUBS)));
    ///
    /// ```
    pub fn insert_card(&mut self, side: Side, card: Card) -> Result<Side, DealError>{
        if self.completed_tricks_number >= QUARTER_SIZE{
            return Err(DealError::DealFull);
        }
        match self.current_trick.add_card_check_exhaust(side, card, &mut self.exhaust_table){
            Ok(4) => {
                match self.current_trick.taker(self.trump()){
                    Ok(winner) => {
                        match self.complete_current_trick(){
                            Ok(()) => Ok(winner),
                            Err(e) => Err(e)

                        }

                    }
                    Err(e) => Err(DealError::TrickError(self.current_trick, e))
                }
            },
            Ok(_) => Ok(side.next()),
            Err(e) => Err(DealError::TrickError(self.current_trick, e))

        }
    }

    pub fn trump(&self) -> Trump{
        self.contract.bid().trump()
    }
    pub fn last_completed_trick(&self) -> Option<&Trick>{
        match self.completed_tricks_number {
            0 => None,
            i @1..=QUARTER_SIZE => Some(&self[i-1]),
            _ => panic!("Deal::Last_trick: deal overflow shouldn't happen")

        }
    }

    pub fn init_new_trick(&self) -> Option<Trick>{
        //println!("{:?}", self.trump());
        match self.last_completed_trick(){
            None => Some(Trick::new(self.contract.owner().prev())),

            Some(t) => t.prepare_new(self.trump())
        }

    }


    /// Based on index of trick returns the side who won the trick.
    /// # Examples: 
    /// ```
    /// use bridge_core::card::suit::Suit::{Clubs, Diamonds, Spades};
    /// use bridge_core::player::side::Side::*;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::card::trump::Trump;
    /// use bridge_core::play::deck::Deck;
    /// use bridge_core::player::side::SIDES;
    /// use bridge_core::play::deal::Deal;
    /// use bridge_core::card::Card;
    /// use bridge_core::card;
    /// use bridge_core::card::figure::{Figure, NumberFigure};
    /// use std::str::FromStr;
    /// use bridge_core::auction::call::Doubling;
    /// use bridge_core::auction::contract::{Contract, Bid};
    /// let deck = Deck::new_sorted_by_figures();
    /// let mut deal_1 = Deal::new(Contract::new_d(North, Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap(), Doubling::None));
    ///
    /// deal_1.insert_card(East, card::KING_SPADES).unwrap();
    /// deal_1.insert_card(South, card::QUEEN_SPADES).unwrap();
    /// deal_1.insert_card(West, card::JACK_SPADES).unwrap();
    /// deal_1.insert_card(North, card::ACE_SPADES).unwrap();
    /// assert_eq!(deal_1.side_winning_trick(0), Ok(North));
    ///
    /// let mut deal_2 = Deal::new(Contract::new_d(West, Bid::create_bid(Trump::NoTrump, 1).unwrap(), Doubling::None));
    ///
    /// deal_2.insert_card(North, card::TWO_DIAMONDS).unwrap();
    /// deal_2.insert_card(East, card::ACE_CLUBS).unwrap();
    /// deal_2.insert_card(South, card::QUEEN_CLUBS).unwrap();
    /// deal_2.insert_card(West, card::THREE_DIAMONDS).unwrap();
    /// assert_eq!(deal_2.side_winning_trick(0), Ok(West));
    /// deal_2.insert_card(West, card::FOUR_DIAMONDS).unwrap();
    /// deal_2.insert_card(North, card::JACK_DIAMONDS).unwrap();
    /// deal_2.insert_card(East, card::KING_CLUBS).unwrap();
    /// deal_2.insert_card(South, card::NINE_SPADES).unwrap();
    /// //deal_2.insert_trick(trick_2_2).unwrap();
    /// assert_eq!(deal_2.side_winning_trick(1), Ok(North));
    /// ```
    pub fn side_winning_trick(&self, index: usize) -> Result<Side, DealError>{
        match index < self.completed_tricks_number {
            true => self[index].taker(self.contract.bid().trump())
                .map_err(|trick_err| DealError::TrickError(self[index], trick_err)),
            false => Err(IndexedOverCurrentTrick(self.completed_tricks_number))
        }
    }
    /// Counts tricks taken by `Side` (one player)
    /// # Examples:
    /// ```
    /// use bridge_core::card::suit::Suit::{*};
    /// use bridge_core::player::side::Side::*;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::card::trump::Trump;
    /// use bridge_core::play::deal::Deal;
    /// use bridge_core::card::Card;
    /// use bridge_core::card;
    /// use bridge_core::card::figure::{Figure, NumberFigure};
    /// use std::str::FromStr;
    /// use bridge_core::auction::contract::{Contract, Bid};
    /// use bridge_core::auction::call::Doubling;
    /// let mut deal = Deal::new(Contract::new(West, Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap() ));
    ///
    /// deal.insert_card(North, card::JACK_SPADES).unwrap();
    /// deal.insert_card(East, card::TEN_SPADES).unwrap();
    /// deal.insert_card(South, card::FOUR_SPADES).unwrap();
    /// deal.insert_card(West, card::FIVE_DIAMONDS).unwrap(); //winner
    ///
    /// deal.insert_card(West, card::EIGHT_HEARTS).unwrap();
    /// deal.insert_card(North, card::JACK_DIAMONDS).unwrap(); //winner
    /// deal.insert_card(East, card::KING_HEARTS).unwrap();
    /// deal.insert_card(South, card::NINE_HEARTS).unwrap();
    ///
    /// deal.insert_card(North, card::ACE_CLUBS).unwrap(); //winner
    /// deal.insert_card(East, card::QUEEN_SPADES).unwrap();
    /// deal.insert_card(South, card::SEVEN_HEARTS).unwrap();
    /// deal.insert_card(West, card::FOUR_CLUBS).unwrap();
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
    /// use bridge_core::card::suit::Suit::{*};
    /// use bridge_core::player::side::Side::*;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::card::trump::Trump;
    /// use bridge_core::play::deal::Deal;
    /// use bridge_core::card::{Card};
    /// use bridge_core::card;
    /// use bridge_core::card::figure::{Figure, NumberFigure};
    /// use std::str::FromStr;
    /// use bridge_core::player::axis::Axis;
    /// use bridge_core::auction::call::Doubling;
    /// use bridge_core::auction::contract::{Contract, Bid};
    /// let mut deal = Deal::new(Contract::new(West, Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap()));
    /// deal.insert_card(North, card::JACK_SPADES).unwrap();
    /// deal.insert_card(East, card::TEN_SPADES).unwrap();
    /// deal.insert_card(South, card::FOUR_SPADES).unwrap();
    /// deal.insert_card(West, card::FIVE_DIAMONDS).unwrap(); //winner
    ///
    /// deal.insert_card(West, card::EIGHT_HEARTS).unwrap();
    /// deal.insert_card(North, card::JACK_DIAMONDS).unwrap(); //winner
    /// deal.insert_card(East, card::KING_HEARTS).unwrap();
    /// deal.insert_card(South, card::NINE_HEARTS).unwrap();
    ///
    /// deal.insert_card(North, card::ACE_CLUBS).unwrap(); //winner
    /// deal.insert_card(East, card::QUEEN_SPADES).unwrap();
    /// deal.insert_card(South, card::SEVEN_HEARTS).unwrap();
    /// deal.insert_card(West, card::FOUR_CLUBS).unwrap();
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

    pub fn score(&self) -> Score{
        let  score = Score::default();


        score
        
    }

}

impl Display for Deal{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClosedDealRubber {
    contract: Deal,
    score: Score

}
/*
impl ClosedDealRubber{

}*/

impl Index<usize> for Deal{
    type Output = Trick;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tricks[index]
    }
}

#[cfg(test)]
mod tests{
    use crate::card;
    use crate::card::suit::Suit::{Diamonds};
    use crate::card::trump::Trump;
    use crate::auction::contract::{Bid, Contract};
    use crate::play::deal::{Deal, DealError};
    use crate::play::deal::DealError::DealFull;
    use crate::play::deck::{Deck, QUARTER_SIZE};
    use crate::player::side::Side;
    use crate::player::side::Side::{East, North, South, West};


    #[test]
    fn deal_duplicate_card(){
        let mut deal = Deal::new(Contract::new(West, Bid::create_bid(Trump::NoTrump, 1).unwrap()));
        let deck = Deck::new_sorted_by_suits();



        deal.insert_card(Side::North, deck[0]).unwrap();
        deal.insert_card(Side::East, deck[2]).unwrap();
        deal.insert_card(Side::South, deck[1]).unwrap();
        deal.insert_card(Side::West, deck[3]).unwrap();

        deal.insert_card(Side::North, deck[0]).unwrap();
        deal.insert_card(Side::East, deck[6]).unwrap();
        deal.insert_card(Side::South, deck[5]).unwrap();

        let r = deal.insert_card(Side::West, deck[7]);


        assert_eq!(r, Err(DealError::DuplicateCard(deck[0])));

    }


    #[test]
    fn deal_overflow_tricks(){
        let num_of_sides = 4usize;
        let deck = Deck::new_sorted_by_suits();
        //let mut deal = Deal::new(South, Trump::NoTrump);
        let mut deal = Deal::new(Contract::new(West, Bid::create_bid(Trump::NoTrump, 1).unwrap()));
        for i in 0..QUARTER_SIZE{

            deal.insert_card(Side::North,deck[num_of_sides*i]).unwrap();
            deal.insert_card(Side::East,deck[num_of_sides*i + 1]).unwrap();
            deal.insert_card(Side::South,deck[num_of_sides*i + 2]).unwrap();
            deal.insert_card(Side::West,deck[num_of_sides*i +3]).unwrap();

        }

        let r = deal.insert_card(Side::North, deck[0]);
        assert_eq!(r, Err(DealFull));



    }

    #[test]
    fn calculate_score_1(){
        let mut deal = Deal::new(Contract::new(
            East,
            Bid::create_bid(Trump::Colored(Diamonds), 3).unwrap()));
        deal.insert_card(South, card::ACE_SPADES).unwrap();
        deal.insert_card(West, card::THREE_SPADES).unwrap();
        deal.insert_card(North, card::FOUR_SPADES).unwrap();
        deal.insert_card(East, card::SIX_SPADES).unwrap();
        assert_eq!(deal.completed_tricks(), 1);

        deal.insert_card(South, card::TWO_SPADES).unwrap();
        deal.insert_card(West, card::THREE_DIAMONDS).unwrap();
        deal.insert_card(North, card::EIGHT_SPADES).unwrap();
        deal.insert_card(East, card::TEN_SPADES).unwrap();
        assert_eq!(deal.completed_tricks(), 2);

        deal.insert_card(West, card::FOUR_DIAMONDS).unwrap();
        deal.insert_card(North, card::JACK_DIAMONDS).unwrap();
        deal.insert_card(East, card::QUEEN_DIAMONDS).unwrap();
        deal.insert_card(South, card::TEN_DIAMONDS).unwrap();

        deal.insert_card(East, card::KING_DIAMONDS).unwrap();
        deal.insert_card(South, card::FIVE_SPADES).unwrap();
        deal.insert_card(West, card::FIVE_DIAMONDS).unwrap();
        deal.insert_card(North, card::NINE_DIAMONDS).unwrap();
        assert_eq!(deal.completed_tricks(), 4);

        deal.insert_card(East, card::FOUR_HEARTS).unwrap();
        deal.insert_card(South, card::FIVE_HEARTS).unwrap();
        deal.insert_card(West, card::KING_HEARTS).unwrap();
        deal.insert_card(North, card::ACE_HEARTS).unwrap();
        assert_eq!(deal.completed_tricks(), 5);

        deal.insert_card(North, card::FIVE_CLUBS).unwrap();
        deal.insert_card(East, card::ACE_CLUBS).unwrap();
        deal.insert_card(South, card::FOUR_CLUBS).unwrap();
        deal.insert_card(West, card::TWO_CLUBS).unwrap();
        assert_eq!(deal.completed_tricks(), 6);

        deal.insert_card(East, card::QUEEN_HEARTS).unwrap();
        deal.insert_card(South, card::EIGHT_HEARTS).unwrap();
        deal.insert_card(West, card::THREE_HEARTS).unwrap();
        deal.insert_card(North, card::TWO_HEARTS).unwrap();

        deal.insert_card(East, card::QUEEN_SPADES).unwrap();
        deal.insert_card(South, card::SEVEN_SPADES).unwrap();
        deal.insert_card(West, card::SIX_DIAMONDS).unwrap();
        deal.insert_card(North, card::NINE_SPADES).unwrap();
        assert_eq!(deal.completed_tricks(), 8);

        deal.insert_card(West, card::KING_CLUBS).unwrap();
        deal.insert_card(North, card::NINE_CLUBS).unwrap();
        deal.insert_card(East, card::TEN_HEARTS).unwrap();
        deal.insert_card(South, card::SIX_CLUBS).unwrap();

        deal.insert_card(West, card::THREE_CLUBS).unwrap();
        deal.insert_card(North, card::QUEEN_CLUBS).unwrap();
        deal.insert_card(East, card::TWO_DIAMONDS).unwrap();
        deal.insert_card(South, card::SEVEN_CLUBS).unwrap();
        assert_eq!(deal.completed_tricks(), 10);

        deal.insert_card(East, card::SEVEN_DIAMONDS).unwrap();
        deal.insert_card(South, card::EIGHT_CLUBS).unwrap();
        deal.insert_card(West, card::SIX_HEARTS).unwrap();
        deal.insert_card(North, card::SEVEN_HEARTS).unwrap();
        assert_eq!(deal.completed_tricks(), 11);

        deal.insert_card(East, card::EIGHT_DIAMONDS).unwrap();
        deal.insert_card(South, card::JACK_CLUBS).unwrap();
        deal.insert_card(West, card::NINE_HEARTS).unwrap();
        deal.insert_card(North, card::JACK_SPADES).unwrap();
        assert_eq!(deal.completed_tricks(), 12);

        deal.insert_card(East, card::ACE_DIAMONDS).unwrap();
        deal.insert_card(South, card::JACK_HEARTS).unwrap();
        deal.insert_card(West, card::TEN_CLUBS).unwrap();
        deal.insert_card(North, card::KING_SPADES).unwrap();


        //assert_eq!(deal.completed_tricks(), 13);
        assert_eq!(deal.tricks_taken(East), 8);
        assert_eq!(deal.tricks_taken(South), 1);
        assert_eq!(deal.tricks_taken(West), 3);
        assert_eq!(deal.tricks_taken(North), 1);










        //trick.

    }





}