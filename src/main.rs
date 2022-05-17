use bridge_core::card::Card;
use bridge_core::card::figure::FigureStd;
use bridge_core::card::suit::SuitStd;
use bridge_core::card::trump::Trump;

use bridge_core::play::trick::Trick;



fn debug_solve_trick(){
    println!("{}", std::mem::size_of::<Trick<FigureStd, SuitStd>>());
    println!("{}", std::mem::size_of::<Card<FigureStd, SuitStd>>());
    println!("{}", std::mem::size_of::<Trump<SuitStd>>());

}

fn main(){
    debug_solve_trick();
}