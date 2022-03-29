use std::fmt::{Display, Formatter};
use std::ops::Index;
use crate::play::trick::{Trick, TrickError};
use crate::player::side::Side;
use serde::{Deserialize, Serialize};
use crate::cards::Card;
use crate::cards::trump::Trump;
use crate::play::axis::Axis;
use crate::play::contract::Contract;
use crate::play::deal::DealError::IndexedOverCurrentTrick;
use crate::play::deck::{LAST_INDEX_IN_DEAL, QUARTER_SIZE};
use crate::play::exhaust::ExhaustTable;
use crate::play::score::Score;
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
    trick_number: usize,
    exhaust_table: ExhaustTable,
    current_trick: Option<Trick>

}
impl Deal{
    pub fn new(contract: Contract) -> Self{
        let first_player = contract.owner().prev();
        Self{contract, tricks: [Trick::new(first_player);QUARTER_SIZE], trick_number: 0,
            exhaust_table: ExhaustTable::default(), current_trick: Some(Trick::new(first_player))}
    }
    pub fn insert_trick(&mut self, trick: Trick) -> Result<(), DealError>{
        match self.trick_number{
            n@0..=LAST_INDEX_IN_DEAL => match trick.missing_card(){
                Some(s) => Err(DealError::TrickError(trick, MissingCard(s))),
                None => {
                    for t in &self.tricks{
                        if let Some(c) = t.collision(&trick) {return Err(DealError::DuplicateCard(c))}
                    }
                    self.tricks[n] = trick;
                    self.trick_number = n+1;
                    Ok(())
                }

            }
            //full if full >= QUARTER_SIZE => Err(DealError::DealFull),
            _ => Err(DealError::DealFull),
        }
    }
    pub fn trump(&self) -> Trump{
        self.contract.bid().trump()
    }
    pub fn last_completed_trick(&self) -> Option<&Trick>{
        match self.trick_number{
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
    /// use bridge_core::cards::suit::Suit::{Clubs, Diamonds, Spades};
    /// use bridge_core::player::side::Side::*;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::play::deck::Deck;
    /// use bridge_core::player::side::SIDES;
    /// use bridge_core::play::deal::Deal;
    /// use bridge_core::cards::Card;
    /// use bridge_core::cards::figure::{Figure, NumberFigure};
    /// use std::str::FromStr;
    /// use bridge_core::play::auction::Doubling;
    /// use bridge_core::play::contract::{Contract, Bid};
    /// let deck = Deck::new_sorted_by_figures();
    /// let mut deal_1 = Deal::new(Contract::new_d(South, Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap(), Doubling::None));
    /// let mut trick_1 = Trick::new(East);
    /// trick_1.add_card(East, Card::from_str("k s").unwrap()).unwrap();
    /// trick_1.add_card(South, Card::from_str("qs").unwrap()).unwrap();
    /// trick_1.add_card(West, Card::from_str("js").unwrap()).unwrap();
    /// trick_1.add_card(North, Card::from_str("a s").unwrap()).unwrap();
    /// deal_1.insert_trick(trick_1).unwrap();
    /// assert_eq!(deal_1.side_winning_trick(0), Ok(North));
    ///
    /// let mut deal_2 = Deal::new(Contract::new_d(East, Bid::create_bid(Trump::NoTrump, 1).unwrap(), Doubling::None));
    /// let mut trick_2_1 = Trick::new(North);
    /// /*
    /// for i in 0..4usize{
    ///     trick_2_1.add_card(SIDES[i], deck[i+40]).unwrap();
    /// }*/
    /// trick_2_1.add_card(North, Card::from_str("2 d").unwrap()).unwrap();
    /// trick_2_1.add_card(East, Card::from_str("ace clubs").unwrap()).unwrap();
    /// trick_2_1.add_card(South, Card::from_str("queen clubs").unwrap()).unwrap();
    /// trick_2_1.add_card(West, Card::from_str("3 d").unwrap()).unwrap();
    ///
    /// let mut trick_2_2 = Trick::new(West);
    ///
    /// trick_2_2.add_card(West, Card::from_str("4 d").unwrap()).unwrap();
    /// trick_2_2.add_card(North, Card::from_str("Jack diamonds").unwrap()).unwrap();
    /// trick_2_2.add_card(East, Card::from_str("king clubs").unwrap()).unwrap();
    /// trick_2_2.add_card(South, Card::from_str("9s").unwrap()).unwrap();
    /// deal_2.insert_trick(trick_2_1).unwrap();
    /// assert_eq!(deal_2.side_winning_trick(0), Ok(West));
    /// deal_2.insert_trick(trick_2_2).unwrap();
    /// assert_eq!(deal_2.side_winning_trick(1), Ok(North));
    /// ```
    pub fn side_winning_trick(&self, index: usize) -> Result<Side, DealError>{
        match index < self.trick_number{
            true => self[index].taker(self.contract.bid().trump())
                .map_err(|trick_err| DealError::TrickError(self[index], trick_err)),
            false => Err(IndexedOverCurrentTrick(self.trick_number))
        }
    }
    /// Counts tricks taken by `Side` (one player)
    /// # Examples:
    /// ```
    /// use bridge_core::cards::suit::Suit::{*};
    /// use bridge_core::player::side::Side::*;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::play::deal::Deal;
    /// use bridge_core::cards::Card;
    /// use bridge_core::cards::figure::{Figure, NumberFigure};
    /// use std::str::FromStr;
    /// use bridge_core::play::contract::{Contract, Bid};
    /// use bridge_core::play::auction::Doubling;
    /// let mut deal = Deal::new(Contract::new(East, Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap() ));
    /// let mut trick_1 = Trick::new(North);
    /// trick_1.add_card(North, Card::from_str("Jack s").unwrap()).unwrap();
    /// trick_1.add_card(East, Card::from_str("10h").unwrap()).unwrap();
    /// trick_1.add_card(South, Card::from_str("4c").unwrap()).unwrap();
    /// trick_1.add_card(West, Card::from_str("5d").unwrap()).unwrap(); //winner
    /// deal.insert_trick(trick_1).unwrap();
    /// let mut trick_2 = trick_1.prepare_new(Trump::Colored(Diamonds)).unwrap();
    /// let trmp = deal.trump();
    /// assert_eq!(trmp, Trump::Colored(Diamonds));
    /// let mut trick_2 = deal.init_new_trick().unwrap();
    /// trick_2.add_card(West, Card::from_str("8s").unwrap()).unwrap();
    /// trick_2.add_card(North, Card::from_str("Jack diamonds").unwrap()).unwrap(); //winner
    /// trick_2.add_card(East, Card::from_str("king clubs").unwrap()).unwrap();
    /// trick_2.add_card(South, Card::from_str("9s").unwrap()).unwrap();
    /// deal.insert_trick(trick_2).unwrap();
    /// //let mut trick_3 = deal.init_new_trick().unwrap();
    /// let mut trick_3 = trick_2.prepare_new(Trump::Colored(Diamonds)).unwrap();
    /// trick_3.add_card(North, Card::from_str("ah").unwrap()).unwrap(); //winner
    /// trick_3.add_card(East, Card::from_str("qs").unwrap()).unwrap();
    /// trick_3.add_card(South, Card::from_str("7h").unwrap()).unwrap();
    /// trick_3.add_card(West, Card::from_str("4s").unwrap()).unwrap();
    /// deal.insert_trick(trick_3).unwrap();
    /// assert_eq!(deal.tricks_taken(North), 2);
    /// assert_eq!(deal.tricks_taken(West), 1);
    /// assert_eq!(deal.tricks_taken(South), 0);
    /// assert_eq!(deal.tricks_taken(East), 0);
    /// ```
    pub fn tricks_taken(&self, side: Side) -> usize{
        self.tricks[0..self.trick_number].iter().filter(|t| t.taker(self.contract.bid().trump()).unwrap() == side).count()
    }
    /// Counts tricks taken by `Side` (one player)
    /// # Examples:
    /// ```
    /// use bridge_core::cards::suit::Suit::{*};
    /// use bridge_core::player::side::Side::*;
    /// use bridge_core::play::trick::Trick;
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::play::deal::Deal;
    /// use bridge_core::cards::Card;
    /// use bridge_core::cards::figure::{Figure, NumberFigure};
    /// use std::str::FromStr;
    /// use bridge_core::play::axis::Axis;
    /// use bridge_core::play::auction::Doubling;
    /// use bridge_core::play::contract::{Contract, Bid};
    /// let mut deal = Deal::new(Contract::new(East, Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap()));
    /// let mut trick_1 = Trick::new(North);
    /// trick_1.add_card(North, Card::from_str("Jack s").unwrap()).unwrap();
    /// trick_1.add_card(East, Card::from_str("10h").unwrap()).unwrap();
    /// trick_1.add_card(South, Card::from_str("4c").unwrap()).unwrap();
    /// trick_1.add_card(West, Card::from_str("5d").unwrap()).unwrap(); //winner
    /// deal.insert_trick(trick_1).unwrap();
    /// let mut trick_2 = Trick::new(West);
    /// trick_2.add_card(West, Card::from_str("8s").unwrap()).unwrap();
    /// trick_2.add_card(North, Card::from_str("Jack diamonds").unwrap()).unwrap(); //winner
    /// trick_2.add_card(East, Card::from_str("king clubs").unwrap()).unwrap();
    /// trick_2.add_card(South, Card::from_str("9s").unwrap()).unwrap();
    /// deal.insert_trick(trick_2).unwrap();
    /// let mut trick_3 = Trick::new(North);
    /// trick_3.add_card(North, Card::from_str("ah").unwrap()).unwrap(); //winner
    /// trick_3.add_card(East, Card::from_str("qs").unwrap()).unwrap();
    /// trick_3.add_card(South, Card::from_str("7h").unwrap()).unwrap();
    /// trick_3.add_card(West, Card::from_str("4s").unwrap()).unwrap();
    /// deal.insert_trick(trick_3).unwrap();
    /// assert_eq!(deal.tricks_taken_axis(Axis::NorthSouth), 2);
    /// assert_eq!(deal.tricks_taken_axis(Axis::EastWest), 1);
    /// ```
    pub fn tricks_taken_axis(&self, axis: Axis) -> usize{
        self.tricks[0..self.trick_number].iter().filter(|t| t.taker(self.contract.bid().trump()).unwrap().axis() == axis).count()
    }

    pub fn is_completed(&self) -> bool{
        match self.trick_number{
            n if n < QUARTER_SIZE => false,
            QUARTER_SIZE => true,
            //Infallible, I guess
            _ => panic!("Number of tricks in deal should never ever exceed {}.", QUARTER_SIZE)
        }
    }

    pub fn score(&self) -> Score{
        let mut score = Score::default();


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
    use crate::cards::suit::Suit;
    use crate::cards::suit::Suit::Diamonds;
    use crate::cards::trump::Trump;
    use crate::cards::trump::Trump::NoTrump;
    use crate::play::auction::Doubling;
    use crate::play::contract::{Bid, Contract};
    use crate::play::deal::{Deal, DealError};
    use crate::play::deal::DealError::DealFull;
    use crate::play::deck::{Deck, QUARTER_SIZE};
    use crate::play::trick::{Trick, TrickError};
    use crate::player::side::Side;
    use crate::player::side::Side::{East, North, South, West};


    #[test]
    fn deal_duplicate_card(){
        let mut deal = Deal::new(Contract::new(East, Bid::create_bid(Trump::NoTrump, 1).unwrap()));
        //let mut deal = Deal::new(South, Trump::NoTrump);
        let mut trick1 = Trick::new(North);
        let deck = Deck::new_sorted_by_suits();

        trick1.add_card(Side::North, deck[0]).unwrap();
        trick1.add_card(Side::East, deck[2]).unwrap();
        trick1.add_card(Side::South, deck[1]).unwrap();
        trick1.add_card(Side::West, deck[3]).unwrap();
        deal.insert_trick(trick1).unwrap();
        let mut trick2 = Trick::new(North);
        trick2.add_card(Side::North, deck[0]).unwrap();
        trick2.add_card(Side::East, deck[6]).unwrap();
        trick2.add_card(Side::South, deck[5]).unwrap();

        trick2.add_card(Side::West, deck[7]).unwrap();

        let r  =deal.insert_trick(trick2);
        assert_eq!(r, Err(DealError::DuplicateCard(deck[0])));

    }

    #[test]
    fn deal_incomplete_trick(){
        //let mut deal = Deal::new(South, Trump::NoTrump);
        let mut deal = Deal::new(Contract::new(East, Bid::create_bid(Trump::NoTrump, 1).unwrap()));
        let mut trick1 = Trick::new(North);
        let deck = Deck::new_sorted_by_suits();
        trick1.add_card(Side::North, deck[0]).unwrap();
        trick1.add_card(Side::East, deck[2]).unwrap();
        trick1.add_card(Side::South, deck[1]).unwrap();
        trick1.add_card(Side::West, deck[3]).unwrap();
        deal.insert_trick(trick1).unwrap();
        let mut trick2 = Trick::new(North);
        trick2.add_card(Side::North, deck[0]).unwrap();

        let r  =deal.insert_trick(trick2);
        assert_eq!(r, Err(DealError::TrickError(trick2, TrickError::MissingCard(East))));
    }

    #[test]
    fn deal_overflow_tricks(){
        let num_of_sides = 4usize;
        let deck = Deck::new_sorted_by_suits();
        //let mut deal = Deal::new(South, Trump::NoTrump);
        let mut deal = Deal::new(Contract::new(East, Bid::create_bid((Trump::NoTrump), 1).unwrap()));
        for i in 0..QUARTER_SIZE{
            let mut trick = Trick::new(North);
            trick.add_card(Side::North, deck[num_of_sides*i]).unwrap();
            trick.add_card(Side::East, deck[num_of_sides*i + 1]).unwrap();
            trick.add_card(Side::South, deck[num_of_sides*i + 2]).unwrap();
            trick.add_card(Side::West, deck[num_of_sides*i + 3]).unwrap();
            deal.insert_trick(trick).unwrap();
        }
        let mut trick = Trick::new(North);
        trick.add_card(Side::North, deck[0]).unwrap();
        trick.add_card(Side::East, deck[1]).unwrap();
        trick.add_card(Side::South, deck[3]).unwrap();
        trick.add_card(Side::West, deck[4]).unwrap();
        let r = deal.insert_trick(trick);
        assert_eq!(r, Err(DealFull));



    }

    #[test]
    fn calculate_score_1(){
        let mut deal = Deal::new(Contract::new(
            South,
            Bid::create_bid(Trump::Colored(Diamonds), 1).unwrap()));

        let mut trick = deal.init_new_trick().unwrap();
        //trick.

    }





}