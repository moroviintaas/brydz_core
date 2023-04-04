//! Module containing basic Symbol trait
//! # Licence:
//! MIT: [https://mit-license.org/](https://mit-license.org/)
//! # Authors:
//! [morovintaas](mailto:moroviintaas@gmail.com)
//!
//!
pub mod player;
pub mod contract;
pub mod score;
pub mod bidding;
pub mod cards;

pub mod meta;
pub mod error;
pub mod deal;

#[cfg(feature = "sztorm")]
pub mod sztorm;


#[cfg(feature = "serde")]
pub use serde;


#[cfg(feature = "speedy")]
pub use karty::speedy;

#[cfg(feature = "serde_ron")]
pub use ron;
pub use karty;

/// Fills contract with cards.
/// # Input:
/// `struct` which is `ContractMaintainer`
/// List of cards in square brackets
/// # Panics:
/// Whenever `insert_card` returns Error
/// # Examples:
/// ```
/// use karty::cards::*;
/// use brydz_core::bidding::consts::BID_H3;
/// use brydz_core::contract::*;
/// use brydz_core::contract::{ContractSpec, ContractMechanics, Contract};
/// use brydz_core::player::side::Side::{East, North, South};
/// use brydz_core::fill_deal;
/// use brydz_core::player::axis::Axis::{EastWest, NorthSouth};
/// let contract = ContractSpec::new(North, BID_H3);
/// let mut dl = Contract::new(contract);
///
/// fill_deal!(dl, [ACE_SPADES, KING_SPADES, QUEEN_SPADES]);
/// assert_eq!(dl.count_completed_tricks(), 0);
/// assert_eq!(dl.current_side(), North);
/// fill_deal!(dl, [JACK_SPADES, THREE_SPADES, NINE_SPADES, EIGHT_SPADES, SEVEN_SPADES]);
/// assert_eq!(dl.total_tricks_taken_axis(EastWest), 1);
/// assert_eq!(dl.count_completed_tricks(), 2);
/// assert_eq!(dl.current_side(), South);
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
#[cfg(all(feature = "serde_derive", feature = "serde_dedicate"))]
compile_error!("features `brydz_core/serde_derive` and `/brydz_core/serde_dedicate` are mutually exclusive");