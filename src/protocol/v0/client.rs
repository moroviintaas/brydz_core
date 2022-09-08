use karty::cards::{CardStd};
use crate::bidding::{ CallStd};
use crate::distribution::hand::BridgeHand;
use crate::error::BridgeErrorStd;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum DealAction{
    PlayCard(CardStd),

}

impl From<DealAction> for ClientDealMessage{
    fn from(m: DealAction) -> Self {
        Self::Action(m)
    }
}
impl From<DealAction> for ClientMessage{
    fn from(m: DealAction) -> Self {
        Self::Deal(m)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum BiddingAction {
    Call(CallStd)
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum DealInfoRequest {
    TODO
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum BiddingInfoRequest {
    TODO
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum ClientDealInformation{
    ShowHand(BridgeHand)
}

impl From<ClientDealInformation> for ClientDealMessage{
    fn from(m: ClientDealInformation) -> Self {
        Self::Info(m)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum ClientDealMessage{
    Action(DealAction),
    Info(ClientDealInformation),
    InfoRequest(DealInfoRequest),
    Control(ClientControlMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum ClientControlMessage{
    IamReady,
    Quit,
    ClientBridgeError(BridgeErrorStd),
    NotMyTurn,
}

impl From<ClientControlMessage> for ClientDealMessage{
    fn from(m: ClientControlMessage) -> Self {
        Self::Control(m)
    }
}

impl From<ClientControlMessage> for ClientMessage{
    fn from(m: ClientControlMessage) -> Self {
        Self::Control(m)
    }
}




#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum ClientMessage{
    Deal(DealAction),
    Bidding(BiddingAction),
    Control(ClientControlMessage),
}
