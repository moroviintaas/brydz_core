
use std::error::Error;
mod channel;
pub use channel::*;


use karty::cards::CardStd;
use crate::player::side::Side;

pub trait OrderGuard{
    fn current_side(&self) -> Option<Side>;
    fn is_dummy_placed(&self) -> bool;
}

pub trait CardCheck<E: Error>{
    fn check(&self, card: &CardStd) -> Result<(), E>;

}

pub trait AutomaticEnvironment<E: Error>{
    fn run(&mut self) -> Result<(), E>;
}
pub trait CommunicatingEnvironment<Sm, Cm, E:Error>{
    fn send(&self, side: &Side, message: Sm) -> Result<(), E>;
    fn send_to_all(&self, message: Sm) -> Result<(), E>;
    fn recv(&self, side: &Side) -> Result<Cm, E>;
    fn try_recv(&self, side: &Side) -> Result<Cm, E>;
}

pub trait StagingEnvironment<E: Error, Sm, Cm>: CommunicatingEnvironment<Sm, Cm, E> {
    fn are_players_ready(&self) -> bool;
    fn run (&mut self) -> Result<(), E>;
    //fn run_until<G: FnMut(&Self) -> bool> (&mut self, guard: G) -> Result<(), E>;
}