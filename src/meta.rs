use karty::figures::Figure;
use karty::suits::Suit;
pub const DECK_SIZE: usize = karty::suits::SuitStd::NUMBER_OF_SUITS * karty::figures::FigureStd::NUMBER_OF_FIGURES;



pub const QUARTER_SIZE: usize = DECK_SIZE/4 ;
pub const MIN_BID_NUMBER: u8 = 1;
pub const HALF_TRICKS: u8 = (QUARTER_SIZE / 2) as u8;
pub const MAX_BID_NUMBER: u8 = QUARTER_SIZE as u8 - HALF_TRICKS;
pub const MAX_INDEX_IN_DEAL: usize = QUARTER_SIZE -1;

