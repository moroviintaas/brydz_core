/*
use karty::figures::Figure;
use karty::suits::Suit;
use crate::contract::deal::{ContractOverseer, DealError};
use crate::error::BridgeError;
use crate::error::BridgeError::Custom;
use crate::player::axis::Axis;
use crate::score::Score;


#[derive(Debug, Copy, Clone)]
pub struct ScoreTableSport {
    ns_score: i32,
    ew_score: i32,
    ns_vulnerability: bool,
    ew_vulnerability: bool

}
impl Default for ScoreTableSport{
    fn default() -> Self {
        Self{ns_score: 0, ew_score: 0, ns_vulnerability: false, ew_vulnerability: false}
    }
}
impl ScoreTableSport{
    pub fn new(ns_vulnerability: bool, ew_vulnerability: bool) -> Self{
        Self{ns_score: 0, ew_score: 0, ns_vulnerability, ew_vulnerability}
    }
    fn points_for_contracted<SuitStd> (contract: &Contract<S>, taken: u8) -> i32{
        let multiiplier = match contract.doubling(){
            Doubling::None => 1,
            Doubling::Double => 2,
            Doubling::ReDouble => 4,
        };
        let base = match contract.bid().trump(){
            Trump::Colored(c) => {
                match c{
                    Diamonds => rewards::tricks::contracted::DIAMONDS,
                    Clubs => rewards::tricks::contracted::CLUBS,

                }
            }
            Trump::NoTrump => {}
        };
    }
}


impl<Co: ContractOverseer<F,S>, F: Figure, S:Suit> Score<Co, F, S>
for ScoreTableSport{

    fn winner_axis(&self) -> Option<Axis> {
        match self.ew_score.cmp(&self.ns_score){
            Ordering::Less => Some(Axis::NorthSouth),
            Ordering::Equal => None,
            Ordering::Greater => Some(Axis::EastWest)
        }
    }

    fn update(&mut self, deal: &Co) -> Result<(), BridgeError<F, S>> {
        if deal.is_completed(){
            let axis = deal.contract().declarer().axis();


            Err(Custom("TODO".to_owned()))
        }
        else{
            Err(BridgeError::DealError(DealError::DealIncomplete))
        }
    }

    fn points(&self, axis: &Axis) -> i32 {
        match axis{
            Axis::EastWest => self.ew_score,
            Axis::NorthSouth => self.ns_score
        }
    }
}

*/