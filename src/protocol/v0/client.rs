use karty::cards::{CardStd};
use crate::bidding::{ CallStd};
use crate::distribution::hand::BridgeHand;
use crate::error::BridgeErrorStd;
use crate::player::side::Side;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealAction{
    PlayCard(CardStd),
    NotMyTurn,
    ShowHand(BridgeHand)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiddingAction {
    Call(CallStd)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealInfoRequest {

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiddingInfoRequest {

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientMessage{
    Dealing(DealAction),
    Bidding(BiddingAction),
    DealInfo(DealInfoRequest),
    BiddingInfo(BiddingInfoRequest),
    Error(BridgeErrorStd),
    Ready,
    Quit,

}
