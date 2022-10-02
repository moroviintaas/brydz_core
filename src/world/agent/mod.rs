mod deal_agent;
mod dummy;

use std::error::Error;
pub use deal_agent::*;
pub use dummy::*;
use crate::distribution::hand::BridgeHand;
use crate::error::BridgeErrorStd;
use crate::protocol::{ClientDealMessage, DealAction, ServerDealMessage};


pub trait Agent<Ac>{
    fn select_action(&self) -> Result<Ac, BridgeErrorStd>;
}

pub trait AwareAgent<S> {
    fn env(&self) -> &S;
    fn env_mut(&mut self) -> &mut S;
    fn set_dummy_hand(&mut self, dummy_hand: BridgeHand);

}

pub trait CommunicatingAgent<SM, CM: From<Ac>, Ac, E:Error> : Agent<Ac>{
    fn send(&mut self, message: CM) -> Result<(),E>;
    fn recv(&mut self) -> Result<SM,E>;

}
pub trait CommunicatingAgentDealStd: CommunicatingAgent<ServerDealMessage, ClientDealMessage, DealAction, BridgeErrorStd>{}

pub trait AutomaticAgent<E: Error>{
    fn run(&mut self) -> Result<(), E>;
}