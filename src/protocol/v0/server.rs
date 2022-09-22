
use karty::cards::CardStd;
use crate::distribution::hand::BridgeHand;
use crate::error::{BridgeErrorStd};
use crate::player::side::Side;

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum DealNotify {
    CardPlayed(Side, CardStd),
    TrickClosed(Side),
    CardAccepted(CardStd),
    CardDeclined(CardStd),
    DummyPlacedHand(BridgeHand),
    YourMove,
    ShowYourHand,
    DealClosed,


}

impl From<DealNotify> for ServerDealMessage{
    fn from(d: DealNotify) -> Self {
        Self::Notify(d)
    }
}

impl From<DealNotify> for ServerMessage{
    fn from(d: DealNotify) -> Self {
        Self::Deal(ServerDealMessage::Notify(d))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum BiddingNotify{
    TODO

}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum DealInfoResponse {
    TODO
}

impl From<DealInfoResponse> for ServerDealMessage{
    fn from(m: DealInfoResponse) -> Self {
        Self::Info(m)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum BiddingInfoResponse{
    TODO
}


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
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
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
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
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum ServerBiddingMessage{
    Notify(BiddingNotify),
    Info(BiddingInfoResponse)
}



#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum ServerMessage{
    Deal(ServerDealMessage),
    Bidding(ServerBiddingMessage),
    Control(ServerControlMessage),
}
