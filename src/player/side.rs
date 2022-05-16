use crate::player::axis::Axis;
use crate::player::axis::Axis::{EastWest, NorthSouth};
use serde::{Deserialize, Serialize};
use crate::player::side::Side::{East, North, South, West};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Side{
    East,
    South,
    West,
    North
}

pub const SIDES : [Side;4] = [North, East, South, West];
impl Side{
    pub fn axis(&self) -> Axis{
        match self{
            Self::East | Self::West=> EastWest,
            Self::North | Self::South => NorthSouth
        }
    }
    pub fn next(&self) -> Self{
        match self{
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East
        }
    }
    pub fn prev(&self) -> Self{
        match self{
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West
        }
    }
    pub fn partner(&self) -> Self{
        match self{
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::North => Self::South
        }
    }
    pub fn next_i(&self, index: u8) -> Self{
        match index & 0x03{
            0 => self.to_owned(),
            1 => self.next(),
            2 => match self{
                North => South,
                East => West,
                South => North,
                West => East
            },
            3 => self.prev(),
            i => {panic!("Next_i {} shouldn't happen", i)}
        }

    }
    pub fn index(&self) -> u8{
        match self{
            North => 0,
            East => 1,
            South => 2,
            West => 3
        }
    }
}