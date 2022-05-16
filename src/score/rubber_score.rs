
use crate::play::deal::Deal;



pub const GAME_WINNING_POINTS: i32 = 100;
pub const RUBBER_WINNING_GAMES: i32 = 2;



#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct Game{
    winning_points: i64,
    contracts: Vec<Deal>,

}
impl Game{
    pub fn new(winning_points: i64) -> Self{
        Self{winning_points, contracts: Vec::new()}
    }

}

/*
impl Game{
    pub fn new() -> Self{
        Self{north_south_scores: Vec::new(), east_west_scores: Vec::new()}
    }
    pub fn sum(&self, axis: Axis) -> i32{
        self[axis].iter().sum()

    }
    pub fn winner(&self) -> Option<Axis>{
        match self.sum(NorthSouth).cmp(&self.sum(EastWest)){
            Ordering::Equal => None,
            Ordering::Greater => Some(NorthSouth),
            Ordering::Less => Some(EastWest)

        }
    }
    pub fn is_finished(&self) -> bool {
        (self.sum(NorthSouth) >= 100) && (self.sum(EastWest) >= 100)
    }
}

impl Default for Game {
     fn default() -> Self {
        Self::new()
     }
}

impl Index<Axis> for Game{
    type Output = Vec<i32>;

    fn index(&self, index: Axis) -> &Self::Output {
        match index{
            Axis::NorthSouth => &self.north_south_scores,
            Axis::EastWest => &self.east_west_scores
        }
    }
}
*/
