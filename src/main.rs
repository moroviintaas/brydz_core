use bridge_core::card::Card;
use bridge_core::card::trump::Trump;

use bridge_core::play::trick::Trick;



fn debug_solve_trick(){
    println!("{}", std::mem::size_of::<Trick>());
    println!("{}", std::mem::size_of::<Card>());
    println!("{}", std::mem::size_of::<Trump>());

}

fn main(){
    debug_solve_trick();
}