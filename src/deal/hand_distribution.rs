use karty::symbol::CardSymbol;
use rand::{prelude::SliceRandom, thread_rng};

use crate::player::side::SideAssociated;

use super::hand::HandTrait;

pub struct HandDistribution{

}



/// Creates fair distribution of cards for bridge game, it assumes that used `CardSymbol` has space divisible by 4;
/// It uses `CardSymbol::iterator()` 
/// ```
/// use brydz_core::deal::fair_bridge_deal;
/// use brydz_core::deal::hand::HandSetStd;
/// use brydz_core::deal::hand::HandTrait;
/// use karty::cards::STANDARD_DECK;
/// let mut table = fair_bridge_deal::<HandSetStd>();
/// assert_eq!(table.north.len(), 13);
/// assert_eq!(table.east.len(), 13);
/// assert_eq!(table.west.len(), 13);
/// assert_eq!(table.south.len(), 13);
/// for c in &STANDARD_DECK{
///     assert!(table.or(|h| h.contains(c)));
/// }
///
/// ```
/// ```
/// use brydz_core::deal::fair_bridge_deal;
/// use brydz_core::deal::hand::StackHand;
/// use brydz_core::deal::hand::HandTrait;
/// use karty::cards::STANDARD_DECK;
/// let mut table = fair_bridge_deal::<StackHand>();
/// assert_eq!(table.north.len(), 13);
/// assert_eq!(table.east.len(), 13);
/// assert_eq!(table.west.len(), 13);
/// assert_eq!(table.south.len(), 13);
/// for c in &STANDARD_DECK{
///     assert!(table.or(|h| h.contains(c)));
/// }
///
/// ```
pub fn fair_bridge_deal<H: HandTrait>() -> SideAssociated<H>{
    let mut result = SideAssociated::<H>{
        north: H::new_empty(),
        east: H::new_empty(),
        south: H::new_empty(),
        west: H::new_empty(),
    };
    let mut rng = thread_rng();
    let mut v  = Vec::from_iter(H::CardType::iterator()); 
    
    v.shuffle(&mut rng);
    let hand_size = v.len()/4;
    /*let north = &v[..hand_size];
    let east = &v[hand_size..2*hand_size];
    let west = &v[2*hand_size..3*hand_size];
    let south = &v[3*hand_size..];
    */

    for _ in 0..hand_size{
        result.north.add_card(v.pop().unwrap()).unwrap();
    }
    for _ in 0..hand_size{
        result.south.add_card(v.pop().unwrap()).unwrap();
    }
    for _ in 0..hand_size{
        result.east.add_card(v.pop().unwrap()).unwrap();
    }
    for _ in 0..hand_size{
        result.west.add_card(v.pop().unwrap()).unwrap();
    }
    result
}