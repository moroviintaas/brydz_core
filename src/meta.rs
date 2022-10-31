use karty::figures::Figure;
use karty::suits::Suit;
use karty::cards::{Card2Sym, CardStd};
pub const DECK_SIZE: usize = karty::suits::SuitStd::NUMBER_OF_SUITS * karty::figures::FigureStd::NUMBER_OF_FIGURES;
pub const HAND_SIZE: usize = CardStd::CARD_SPACE/4;


pub const QUARTER_SIZE: usize = DECK_SIZE/4 ;
pub const MIN_BID_NUMBER: u8 = 1;

pub const SIZE_SMALLER_HALF_TRICKS: usize = QUARTER_SIZE/2;
pub const SIZE_GREATER_HALF_TRICKS: usize = QUARTER_SIZE - SIZE_SMALLER_HALF_TRICKS;
pub const HALF_TRICKS: u8 = (QUARTER_SIZE / 2) as u8;
pub const MAX_BID_NUMBER: u8 = QUARTER_SIZE as u8 - HALF_TRICKS;
pub const MAX_INDEX_IN_DEAL: usize = QUARTER_SIZE -1;

