use std::error::Error;
use crate::world::environment::{CommunicatingEnvironment};
//, F: FnMut(&card) -> Result<(),E>
pub trait StagingEnvironment<E: Error, Sm, Cm>: CommunicatingEnvironment<Sm, Cm, E> {
    fn are_players_ready(&self) -> bool;
    fn run (&mut self) -> Result<(), E>;
    //fn run_until<G: FnMut(&Self) -> bool> (&mut self, guard: G) -> Result<(), E>;
}
/*
impl<E: Error, Sm, Cm, T> AutomaticEnvironment<E> for T
where T: StagingEnvironment<E, Sm, Cm> + OrderGuard{
    fn run(&mut self) -> Result<(), E> {
        info!("Waiting for all players to be ready.");
        self.run_until(|env| env.are_players_ready())?;
        if let Some(whist) = self.current_side(){
            self.send(&whist, YourMove.into())?;
            self.run_until(|env| self.is_dummy_placed())?;
            self.run_until(|_| true)
        }
        Err(DealFull.into())

    }
}

 */