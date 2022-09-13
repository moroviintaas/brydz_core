use brydz_core::deal::Trick;
use brydz_core::cards::trump::Trump;
use karty::cards::Card;
use karty::figures::FigureStd;
use karty::suits::SuitStd;


fn debug_solve_trick(){
    println!("{}", std::mem::size_of::<Trick<FigureStd, SuitStd>>());
    println!("{}", std::mem::size_of::<Card<FigureStd, SuitStd>>());
    println!("{}", std::mem::size_of::<Trump<SuitStd>>());

}

fn main(){
    debug_solve_trick();
}