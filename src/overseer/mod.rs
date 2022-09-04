mod simple;
mod status;

pub use simple::*;
pub use status::*;
use crate::error::{ BridgeErrorStd};

pub trait Overseer{
    fn run(&mut self) -> Result<(), BridgeErrorStd>;
}