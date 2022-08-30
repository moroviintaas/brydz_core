use std::fmt::{Debug, Display};
use std::error::Error;

#[derive(Debug, Clone)]
pub enum Correctness<Entry: Debug + Display + Clone, E: Error>{
    Correct(Entry),
    Wrong(Entry, E)

}