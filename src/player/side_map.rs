use crate::player::side::{Side, SIDES};
use crate::player::side::Side::{East, North, South, West};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

/// ```
/// use brydz_core::player::side::SideMap;
/// use karty::cards::Card;
/// assert_eq!(std::mem::size_of::<SideMap<Card>>(), 12)
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub struct SideMap<T>{
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T
}

impl<T> SideMap<T>{
    pub fn new(north: T, east: T, south: T, west:T) -> Self{
        Self{north, east, south, west}
    }
    pub fn new_with_fn<F>(f: F) -> Self where F: Fn(Side) -> T{
        Self{
            north: f(North),
            east: f(East),
            south: f(South),
            west: f(West),
        }
    }
    pub fn new_symmetric(sym: T)  -> Self where T: Clone{
        Self{north: sym.clone(), east: sym.clone(), south: sym.clone(), west: sym }
    }
    pub fn and<F: Fn(&T) -> bool >(&self, f:F) -> bool{
        f(&self.north) && f(&self.east) && f(&self.south) && f(&self.west)
    }
    pub fn or<F: Fn(&T) -> bool + Copy>(&self, f:F) -> bool{
        f(&self.north) || f(&self.east) || f(&self.south) || f(&self.west)
    }
    pub fn transform<D, F: FnOnce(&T) -> D + Copy>(&self, f: F) -> SideMap<D>{
        SideMap {north: f(&self.north), south: f(&self.south), east: f(&self.east), west: f(&self.west)}
    }
    pub fn find<F: FnOnce(&T) -> bool + Copy>(&self, f: F) -> Option<Side>{
        for s in SIDES{
            if f(&self[&s]){
                return Some(s)
            }
        }
        None
    }

    pub fn merge<F: Fn(&T, &T) -> T> (&self, f:F) -> T{
        let ns = f(&self.north, &self.south);
        let we = f(&self.west, &self.east);
        f(&ns, &we)
    }

    pub fn destruct(self) -> (T,T,T,T){
    (self.north, self.east, self.south, self.west)
    }
    pub fn select_best_fit<C: Ord, F: Fn(&T) -> C>(&self, fit: F) -> Side{
        let mut max = fit(&self.north);
        let mut best = North;
        for side in &SIDES[0..]{
            let tmp = fit(&self[side]);
            if  tmp > max{
                best = *side;
                max = tmp;
            }
        }
        best
    }

}

impl<T: Eq> SideMap<T>{
    pub fn are_all_equal(&self) -> bool{
        let t_north = &self.north;
        self.and(|c| c== t_north)
    }
}