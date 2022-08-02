use karty::cards::Card;
use karty::figures::Figure;
use karty::suits::Suit;
use crate::deal::trick::{Trick};
use crate::player::side::Side;
use crate::player::axis::Axis;
use crate::deal::contract::Contract;
use crate::error::DealError;


pub trait DealMaintainer<F: Figure, S: Suit>{
    fn current_trick(&self) -> &Trick<F, S>;
    fn contract(&self) -> &Contract<S>;
    fn count_completed_tricks(&self) -> usize;
    fn insert_card(&mut self, side: Side, card: Card<F, S>) -> Result<Side, DealError<F, S>>;
    fn is_completed(&self) -> bool;
    fn completed_tricks(&self) -> Vec<Trick<F,S>>;
    fn total_tricks_taken_side(&self, side: Side) -> usize;
    fn total_tricks_taken_axis(&self, axis: Axis) -> usize;
    fn current_side(&self) -> Option<Side>{
        self.current_trick().current_side()
    }
}

#[macro_export]
macro_rules! fill_deal {
    ($deal:ident,  [$($card:ident),*] ) => {
        {
            let _s = $deal.current_trick().current_side().unwrap();
            $(

                $deal.insert_card(_s, $card).unwrap();
                let _s = _s.next();
            )*
        };

    }
}

#[cfg(test)]
mod tests{
    use karty::cards::{ACE_SPADES, EIGHT_SPADES, JACK_SPADES, KING_SPADES, NINE_SPADES, QUEEN_SPADES, SEVEN_SPADES, TEN_SPADES};
    use crate::bidding::consts::BID_H3;
    use crate::deal::{Contract, DealMaintainer, RegDealStd};
    use crate::player::side::Side::{East, North};

    #[test]
    fn macro_fill_deal(){
        let contract = Contract::new(North, BID_H3);
        let mut dl = RegDealStd::new(contract);
        fill_deal!(dl, [ACE_SPADES, KING_SPADES, QUEEN_SPADES]);
        assert_eq!(dl.count_completed_tricks(), 0);
        assert_eq!(dl.current_side(), Some(North));
        fill_deal!(dl, [JACK_SPADES, TEN_SPADES, NINE_SPADES, EIGHT_SPADES, SEVEN_SPADES]);
        assert_eq!(dl.count_completed_tricks(), 2);
        assert_eq!(dl.current_side(), Some(East));
    }
}