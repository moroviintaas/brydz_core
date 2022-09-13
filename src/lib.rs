pub mod player;
pub mod deal;
pub mod score;
pub mod bidding;
pub mod cards;

pub mod meta;
pub mod error;
pub mod distribution;

#[cfg(feature = "protocol")]
pub mod protocol;
#[cfg(feature = "world")]
pub mod world;


#[cfg(feature = "speedy")]
pub use karty::speedy;


pub use ron;
pub use karty;

/// Fills deal with cards.
/// # Input:
/// `struct` which is `DealMaintainer`
/// List of cards in square brackets
/// # Panics:
/// Whenever `insert_card` returns Error
/// # Examples:
/// ```
/// use karty::cards::*;
/// use brydz_core::bidding::consts::BID_H3;
/// use brydz_core::deal::*;
/// use brydz_core::deal::{Contract, DealMaintainer, RegDealStd};
/// use brydz_core::player::side::Side::{East, North, South};
/// use brydz_core::fill_deal;
/// use brydz_core::player::axis::Axis::{EastWest, NorthSouth};
/// let contract = Contract::new(North, BID_H3);
/// let mut dl = RegDealStd::new(contract);
///
/// fill_deal!(dl, [ACE_SPADES, KING_SPADES, QUEEN_SPADES]);
/// assert_eq!(dl.count_completed_tricks(), 0);
/// assert_eq!(dl.current_side(), Some(North));
/// fill_deal!(dl, [JACK_SPADES, THREE_SPADES, NINE_SPADES, EIGHT_SPADES, SEVEN_SPADES]);
/// assert_eq!(dl.total_tricks_taken_axis(EastWest), 1);
/// assert_eq!(dl.count_completed_tricks(), 2);
/// assert_eq!(dl.current_side(), Some(South));
/// ```
#[macro_export]
macro_rules! fill_deal {
    ($deal:ident,  [$($card:ident),*] ) => {
        {

            $(
                let _s = $deal.current_trick().current_side().unwrap();
                $deal.insert_card(_s, $card).unwrap();
            )*
        };

    }
}