
use std::error::Error;

mod channel;
mod deal_env_rr;
pub use deal_env_rr::*;

pub use channel::*;


use karty::cards::CardStd;
use crate::deal::{ RegDealStd};
use crate::distribution::hand::BridgeHand;
use crate::error::{BridgeErrorStd};
use crate::player::side::{Side};
pub trait OrderGuard{
    fn current_side(&self) -> Option<Side>;
    fn is_dummy_placed(&self) -> bool;
}

pub trait CardCheck<E: Error>{
    fn check(&self, card: &CardStd) -> Result<(), E>;

}

pub trait Environment{
    fn deal(&self) -> &RegDealStd;
    fn deal_mut(&mut self) -> &mut RegDealStd;
    fn card_check(&self, card: &CardStd) -> Result<(), BridgeErrorStd>;
    fn dummy_hand(&self) -> Option<&BridgeHand>;
    fn set_dummy_hand(&mut self, hand: BridgeHand);
    fn next_player(&self) -> Option<Side>;
}

pub trait WaitReady{
    fn are_players_ready(&self) -> bool;
    fn set_ready(&mut self, side: &Side);
}
/*
pub trait AutomaticEnvironment<E: Error>{
    fn run(&mut self) -> Result<(), E>;
}*/
pub trait CommunicatingEnvironment<Sm, Cm, E:Error>{
    fn send(&self, side: &Side, message: Sm) -> Result<(), E>;
    fn send_to_all(&self, message: Sm) -> Result<(), E>;
    fn recv(&mut self, side: &Side) -> Result<Cm, E>;
    fn try_recv(&mut self, side: &Side) -> Result<Cm, E>;
}
