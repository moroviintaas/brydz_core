mod deal_agent;

use std::error::Error;
pub use deal_agent::*;
use crate::error::BridgeErrorStd;


pub trait Agent<Ac>{
    fn select_action(&self) -> Result<Ac, BridgeErrorStd>;
}

pub trait AwareAgent<S> {
    fn env(&self) -> &S;
    fn env_mut(&mut self) -> &mut S;

}

pub trait CommunicatingAgent<SM, CM: From<Ac>, Ac, E:Error> : Agent<Ac>{
    fn send(&self, message: CM) -> Result<(),E>;
    fn recv(&self) -> Result<SM,E>;

}

pub trait AutomaticAgent<E: Error>{
    fn run(&mut self) -> Result<(), E>;
}