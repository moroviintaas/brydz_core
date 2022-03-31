use serde::{Deserialize, Serialize};

use std::cmp::Ordering;
use crate::cards::figure::Figure::{Ace, King, Queen, Jack, Numbered};
pub const MAX_NUMBER_FIGURE: u8 = 10;
pub const MIN_NUMBER_FIGURE: u8 = 2;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Hash)]
pub struct NumberFigure{
    power: u8
}
impl NumberFigure{
    pub fn new(power: u8) -> Self{
        match power{
            legit @MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE => Self{power: legit},
            e => panic!("Invalid power value {:?}", e)
        }
    }
    pub fn power(&self) -> u8{
        self.power
    }
}

pub const F2:NumberFigure = NumberFigure{power: 2};
pub const F3:NumberFigure = NumberFigure{power: 3};
pub const F4:NumberFigure = NumberFigure{power: 4};
pub const F5:NumberFigure = NumberFigure{power: 5};
pub const F6:NumberFigure = NumberFigure{power: 6};
pub const F7:NumberFigure = NumberFigure{power: 7};
pub const F8:NumberFigure = NumberFigure{power: 8};
pub const F9:NumberFigure = NumberFigure{power: 9};
pub const F10:NumberFigure = NumberFigure{power: 10};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Hash)]
pub enum Figure{
    Ace,
    King,
    Queen,
    Jack,
    Numbered(NumberFigure)
}
impl Figure{
    fn order(&self) -> u8{
        match self{
            Ace => 14,
            King=> 13,
            Queen=> 12,
            Jack=> 11,
            Numbered(fig) => fig.power()
        }

    }

}

pub const FIGURES: [Figure;13] = [Ace, King, Queen, Jack, Numbered(NumberFigure{power: 10}),
        Numbered(NumberFigure{power: 9}), Numbered(NumberFigure{power: 8}),
        Numbered(NumberFigure{power: 7}), Numbered(NumberFigure{power: 6}),
        Numbered(NumberFigure{power: 5}), Numbered(NumberFigure{power: 4}),
        Numbered(NumberFigure{power: 3}), Numbered(NumberFigure{power: 2})];

impl PartialOrd for Figure{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.order().cmp(&other.order()))
    }
}

impl Ord for Figure{
    fn cmp(&self, other: &Self) -> Ordering {
        self.order().cmp(&other.order())
    }
}


#[cfg(test)]
mod tests{
    use crate::cards::figure::{NumberFigure, Figure};
    #[test]
    fn test_ordering(){
        let king = Figure::King;
        let ten = Figure::Numbered(NumberFigure::new(10));
        let four = Figure::Numbered(NumberFigure::new(4));
        let ace = Figure::Ace;
        let king2 = Figure::King;

        assert!(king > ten);
        assert!(four < ten);
        assert!(king < ace);

        assert_eq!(king, king2);
    }
}




