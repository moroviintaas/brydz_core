
use karty::cards::CardStd;
use crate::distribution::hand::BridgeHand;
use crate::error::{BridgeErrorStd};
use crate::player::side::Side;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealNotify {
    CardPlayed(Side, CardStd),
    TrickClosed(Side),
    CardAccepted(CardStd),
    CardDeclined(CardStd),
    DummyPlacedHand(BridgeHand),
    YourMove,
    DealClosed,


}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiddingNotify{

}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealInfoResponse {

}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiddingInfoResponse{

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerMessage{
    Deal(DealNotify),
    Bidding(BiddingNotify),
    PlayerLeft(Side),
    DealInfo(DealInfoResponse),
    BiddingInfo(DealInfoResponse),
    GameOver,
    ServerStopping,

    ServerNotReady,
    Error(BridgeErrorStd),


}
