use std::cmp::Ordering;
use karty::figures::{Figure};
use karty::suits::{SuitStd};
use crate::deal::{DealMaintainer};
use crate::error::{BridgeError, DealError};
use crate::player::axis::Axis;
use crate::score::calculation::ScoreIngredient;
use crate::score::ScoreTracker;
use crate::score::tables::{PENALTY_UNDER_TRICK, POINTS_CONTRACTED_TRICK, POINTS_OVER_TRICK, POINTS_PREMIUM_CONTRACT, POINTS_PREMIUM_SPORT, POINTS_SLAM};


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

    /// # Example:
    /// ```
    /// todo!();
    /// assert!(false);
    /// ```
    fn update(&mut self, deal: &Co) -> Result<(), BridgeError<F, SuitStd>> {
        if deal.is_completed(){
            let axis = deal.contract().declarer().axis();
            let vulnerability = match axis{
                Axis::EastWest => self.ew_vulnerability,
                Axis::NorthSouth => self.ns_vulnerability
            };
            let defender_vulnerability = match axis{
                Axis::EastWest => self.ns_vulnerability,
                Axis::NorthSouth => self.ew_vulnerability
            };
            let taken = deal.total_tricks_taken_axis(axis) as u8;
            let contracted_points = POINTS_CONTRACTED_TRICK.calculate(deal.contract(), taken, false);
            let overtrick_bonus = POINTS_OVER_TRICK.points(deal.contract(), taken, vulnerability);
            let slam_bonus = POINTS_SLAM.points(deal.contract(), taken, vulnerability);
            let premium_game_points = POINTS_PREMIUM_SPORT.points(contracted_points, vulnerability);
            let premium_contract_points = POINTS_PREMIUM_CONTRACT.points(deal.contract(), taken);
            let penalty_undertricks = match PENALTY_UNDER_TRICK.penalty_checked(deal.contract(), taken, defender_vulnerability){
                Ok(points) => points,
                Err(e) => return Err(BridgeError::Score(e))
            };

            let declarer_axis_score = contracted_points+ overtrick_bonus + slam_bonus
                + premium_game_points + premium_contract_points;
            let defender_axis_score = penalty_undertricks;

            match axis{
                Axis::NorthSouth => {
                    self.ns_score += declarer_axis_score;
                    self.ew_score += defender_axis_score;
                }
                Axis::EastWest => {
                    self.ns_score += defender_axis_score;
                    self.ew_score += declarer_axis_score;
                }
            }
            Ok(())


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

