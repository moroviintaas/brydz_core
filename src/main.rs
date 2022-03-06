use bridge_core::card::suit::Suit::Hearts;
use bridge_core::card::trump::Trump::Colored;
use bridge_core::deck::Deck;
use bridge_core::player::role::PlayRole::{Declarer, Dummy, FirstDefender, SecondDefender};
use bridge_core::table::trick::Trick;

fn debug_solve_trick(){
    let mut trick1 = Trick::new(Colored(Hearts));
    let deck = Deck::new_id_rand();
    let qh = deck[9]; //queen of hearts
    let ap = deck[0]; //ace of spades
    let tens = deck[16]; //ten of spades
    let twoc = deck[51]; //two of clubs

    println!("{:?}", &deck);
    println!("{:?}, {:?}, {:?}, {:?}", &qh, &ap, &tens, &twoc);
    trick1.add_card(FirstDefender, qh).unwrap();
    trick1.add_card(SecondDefender, ap).unwrap();
    trick1.add_card(Declarer, tens).unwrap();
    trick1.add_card(Dummy, twoc).unwrap();

    println!("{:?}", trick1);

}

fn main(){
    debug_solve_trick();
}