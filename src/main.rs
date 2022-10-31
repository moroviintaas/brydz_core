use brydz_core::contract::Trick;
use brydz_core::cards::trump::Trump;
use karty::cards::{Card, CardStd};
use karty::figures::FigureStd;
use karty::suits::SuitStd;


fn debug_solve_trick(){
    println!("{}", std::mem::size_of::<Trick<CardStd>>());
    println!("{}", std::mem::size_of::<Card<FigureStd, SuitStd>>());
    println!("{}", std::mem::size_of::<Trump<SuitStd>>());

}

fn main(){
    debug_solve_trick();
}