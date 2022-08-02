use std::cmp::Ordering;
use karty::figures::{Figure};
use karty::suits::{SuitStd};
use crate::deal::{DealMaintainer};
use crate::error::{BridgeError, DealError};
use crate::error::BridgeError::Custom;
use crate::player::axis::Axis;
use crate::score::calculation::ScoreIngredient;
use crate::score::ScoreTracker;
use crate::score::tables::{POINTS_CONTRACTED_TRICK, POINTS_OVER_TRICK, POINTS_SLAM};


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
    /*
    fn points_for_contracted<SuitStd> (deal: &Contract<S>, taken: u8) -> i32{
        let multiiplier = match deal.doubling(){
            Doubling::None => 1,
            Doubling::Double => 2,
            Doubling::ReDouble => 4,
        };
        let base = match deal.bid().trump(){
            Trump::Colored(c) => {
                match c{
                    Diamonds => rewards::tricks::contracted::DIAMONDS,
                    Clubs => rewards::tricks::contracted::CLUBS,

                }
            }
            Trump::NoTrump => {}
        };
    }

     */
}


impl<Co: DealMaintainer<F,SuitStd>, F: Figure> ScoreTracker<Co, F, SuitStd>
for ScoreTableSport{

    fn winner_axis(&self) -> Option<Axis> {
        match self.ew_score.cmp(&self.ns_score){
            Ordering::Less => Some(Axis::NorthSouth),
            Ordering::Equal => None,
            Ordering::Greater => Some(Axis::EastWest)
        }
    }

    fn update(&mut self, deal: &Co) -> Result<(), BridgeError<F, SuitStd>> {
        if deal.is_completed(){
            let axis = deal.contract().declarer().axis();
            let vulnerability = match axis{
                Axis::EastWest => self.ew_vulnerability,
                Axis::NorthSouth => self.ns_vulnerability
            };
            let taken = deal.total_tricks_taken_axis(axis) as u8;
            let contracted_points = POINTS_CONTRACTED_TRICK.calculate(deal.contract(), taken, false);
            let overtrick_bonus = POINTS_OVER_TRICK.points(deal.contract(), taken, vulnerability);
            let slam_bonus = POINTS_SLAM.points(deal.contract(), taken, vulnerability);
            todo!();

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

