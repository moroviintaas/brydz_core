use serde::{Deserialize, Serialize};

use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use crate::card::figure::FigureStd::{Ace, King, Queen, Jack, Numbered};
pub const MAX_NUMBER_FIGURE: u8 = 10;
pub const MIN_NUMBER_FIGURE: u8 = 2;


pub trait Figure: Debug + Eq + Ord + Clone{
    const NUMBER_OF_FIGURES: u8;
    fn power(&self) -> u8;
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Hash)]
pub struct NumberFigureStd {
    power: u8
}
impl NumberFigureStd {
    pub fn new(power: u8) -> Self{
        match power{
            legit @MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE => Self{power: legit},
            e => panic!("Invalid power value {:?}", e)
        }
    }

    /// Returns a mask of a figure in manner:
    ///
    /// ```
    /// use bridge_core::card::figure;
    /// assert_eq!(figure::F2.mask(), 0x04);
    /// assert_eq!(figure::F3.mask(), 0x08);
    /// assert_eq!(figure::F4.mask(), 0x10);
    /// assert_eq!(figure::F5.mask(), 0x20);
    /// assert_eq!(figure::F6.mask(), 0x40);
    /// assert_eq!(figure::F7.mask(), 0x80);
    /// assert_eq!(figure::F8.mask(), 0x100);
    /// assert_eq!(figure::F9.mask(), 0x200);
    /// assert_eq!(figure::F10.mask(), 0x400);
    ///
    /// ```
    pub fn mask(&self) -> u64{
        1u64<<self.power
    }
}

impl Ord for NumberFigureStd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.power.cmp(&other.power)
    }
}

impl PartialOrd<Self> for NumberFigureStd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.power.partial_cmp(&other.power)
    }
}

impl Figure for NumberFigureStd{
    const NUMBER_OF_FIGURES: u8 = 9;

    fn power(&self) -> u8{
        self.power
    }
}

pub const F2: NumberFigureStd = NumberFigureStd {power: 2};
pub const F3: NumberFigureStd = NumberFigureStd {power: 3};
pub const F4: NumberFigureStd = NumberFigureStd {power: 4};
pub const F5: NumberFigureStd = NumberFigureStd {power: 5};
pub const F6: NumberFigureStd = NumberFigureStd {power: 6};
pub const F7: NumberFigureStd = NumberFigureStd {power: 7};
pub const F8: NumberFigureStd = NumberFigureStd {power: 8};
pub const F9: NumberFigureStd = NumberFigureStd {power: 9};
pub const F10: NumberFigureStd = NumberFigureStd {power: 10};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Hash)]
pub enum FigureStd {
    Ace,
    King,
    Queen,
    Jack,
    Numbered(NumberFigureStd)
}
impl FigureStd {

    /// Returns a mask for figure for efficient storing bool tables
    /// ```
    /// use bridge_core::card::figure;
    /// use bridge_core::card::figure::FigureStd;
    /// assert_eq!(FigureStd::Ace.mask(),      0b0100000000000000);
    /// assert_eq!(FigureStd::King.mask(),     0b0010000000000000);
    /// assert_eq!(FigureStd::Queen.mask(),    0b0001000000000000);
    /// assert_eq!(FigureStd::Jack.mask(),     0b0000100000000000);
    /// assert_eq!(figure::F10.mask(),      0b0000010000000000);
    /// assert_eq!(figure::F2.mask(),       0b0000000000000100);
    /// ```
    pub fn mask(&self) -> u64{
        match self{
            FigureStd::Ace => 0x4000,
            FigureStd::King => 0x2000,
            FigureStd::Queen => 0x1000,
            FigureStd::Jack => 0x800,
            Numbered(n) => n.mask()

        }
    }

}
impl Figure for FigureStd{
    const NUMBER_OF_FIGURES: u8 = 13;
    fn power(&self) -> u8{
        match self{
            Ace => 14,
            King=> 13,
            Queen=> 12,
            Jack=> 11,
            Numbered(fig) => fig.power()
        }
    }
}

pub const FIGURES: [FigureStd;13] = [Ace, King, Queen, Jack, Numbered(NumberFigureStd {power: 10}),
        Numbered(NumberFigureStd {power: 9}), Numbered(NumberFigureStd {power: 8}),
        Numbered(NumberFigureStd {power: 7}), Numbered(NumberFigureStd {power: 6}),
        Numbered(NumberFigureStd {power: 5}), Numbered(NumberFigureStd {power: 4}),
        Numbered(NumberFigureStd {power: 3}), Numbered(NumberFigureStd {power: 2})];

impl PartialOrd for FigureStd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.power().cmp(&other.power()))
    }
}

impl Ord for FigureStd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.power().cmp(&other.power())
    }
}


#[cfg(test)]
mod tests{
    use crate::card::figure::{NumberFigureStd, FigureStd};
    #[test]
    fn test_ordering(){
        let king = FigureStd::King;
        let ten = FigureStd::Numbered(NumberFigureStd::new(10));
        let four = FigureStd::Numbered(NumberFigureStd::new(4));
        let ace = FigureStd::Ace;
        let king2 = FigureStd::King;

        assert!(king > ten);
        assert!(four < ten);
        assert!(king < ace);

        assert_eq!(king, king2);
    }
}




