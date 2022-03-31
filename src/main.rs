use bridge_core::cards::Card;
use bridge_core::cards::figure::Figure::{Ace, Numbered, Queen};
use bridge_core::cards::figure::NumberFigure;
use bridge_core::cards::suit::Suit::{Clubs, Hearts, Spades};
use bridge_core::play::deck::Deck;
use bridge_core::play::trick::Trick;
use bridge_core::player::side::Side::{East, North, South, West};

fn debug_solve_trick(){
    let mut trick1 = Trick::new(North);
    let deck = Deck::new_sorted_by_suits();
    let qh = Card::new(Queen, Hearts);
    let ap = Card::new(Ace, Spades);
    let tens = Card::new(Numbered(NumberFigure::new(10)), Spades);
    let twoc = Card::new(Numbered(NumberFigure::new(2)), Clubs);

    println!("{:?}", &deck);
    println!("{:?}, {:?}, {:?}, {:?}", &qh, &ap, &tens, &twoc);
    trick1.add_card(North, qh).unwrap();
    trick1.add_card(South, ap).unwrap();
    trick1.add_card(West, tens).unwrap();
    trick1.add_card(East, twoc).unwrap();

    println!("{:?}", trick1);
    for i in 0..3{
        println!("{}", i);
    }

}

fn main(){
    debug_solve_trick();
}