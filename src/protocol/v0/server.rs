
use karty::cards::CardStd;
use crate::error::{BridgeErrorStd};
use crate::player::side::Side;

pub enum DealNotify {
    CardPlayed(Side, CardStd),
    TrickClosed(Side),
    YourMove,
    YourMoveDummy,
    CardAccepted(CardStd),


}

pub enum BiddingNotify{

}

pub enum ServerMessage{
    Deal(DealNotify),
    Bidding(BiddingNotify),
    PlayerLeft(Side),
    GameOver,
    Error(BridgeErrorStd),


}
