use bridge_core::play::trick::Trick;
use bridge_core::play::trump::Trump;
use carden::cards::Card;
use carden::figures::FigureStd;
use carden::suits::SuitStd;


fn debug_solve_trick(){
    println!("{}", std::mem::size_of::<Trick<FigureStd, SuitStd>>());
    println!("{}", std::mem::size_of::<Card<FigureStd, SuitStd>>());
    println!("{}", std::mem::size_of::<Trump<SuitStd>>());

}

fn main(){
    debug_solve_trick();
}