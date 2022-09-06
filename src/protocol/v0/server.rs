
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

impl From<DealNotify> for ServerDealMessage{
    fn from(d: DealNotify) -> Self {
        Self::Notify(d)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiddingNotify{

}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealInfoResponse {

}

impl From<DealInfoResponse> for ServerDealMessage{
    fn from(m: DealInfoResponse) -> Self {
        Self::Info(m)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiddingInfoResponse{

}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerControlMessage{
    ServerStopping,
    ServerBridgeError(BridgeErrorStd),
    ServerNotReady,
    PlayerLeft(Side),
    GameOver,
    GameOverUnfinished
}

impl From<ServerControlMessage> for ServerDealMessage{
    fn from(m: ServerControlMessage) -> Self {
        Self::Control(m)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerDealMessage{
    Notify(DealNotify),
    Info(DealInfoResponse),
    Control(ServerControlMessage)

}

impl From<ServerDealMessage> for ServerMessage{
    fn from(m: ServerDealMessage) -> Self {
        Self::Deal(m)
    }
}




#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerBiddingMessage{
    Notify(BiddingNotify),
    Info(BiddingInfoResponse)
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerMessage{
    Deal(ServerDealMessage),
    Bidding(ServerBiddingMessage),
    Control(ServerControlMessage),
}
