use karty::cards::{Card};
use crate::bidding::Bid;
use crate::player::side::Side;
use crate::karty::figures::Figure;
use crate::karty::suits::Suit;



pub enum DealActionGeneric<F: Figure, S:Suit> {
    PlayCard(Side, Card<F,S>),
    Quit
}
pub enum BiddingAction<S: Suit> {
    Bid(Bid<S>)
}