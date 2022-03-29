use serde::{Deserialize, Serialize};
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum Axis{
    NorthSouth,
    EastWest
}