use karty::suits::SuitStd;
use crate::bidding::call::Doubling;
use crate::bidding::contract::Contract;
use crate::cards::trump::Trump;

pub struct PointsOverTrick{
    pub not_doubled_clubs: i32,
    pub not_doubled_diamonds: i32,
    pub not_doubled_hearts: i32,
    pub not_doubled_spades: i32,
    pub not_doubled_nt: i32,
    pub doubled_not_vulnerable: i32,
    pub doubled_vulnerable: i32,
    pub redoubled_not_vulnerable: i32,
    pub redoubled_vulnerable: i32,
}

impl PointsOverTrick{
    /// Calculates points for taken overtricks.
    /// # Examples:
    /// ```
    /// use bridge_core::bidding::contract::Contract;
    /// use bridge_core::player::side::Side::North;
    /// use bridge_core::bidding::bid::Bid;
    /// use bridge_core::bidding::call::Doubling::{ReDouble, Double};
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::cards::trump::Trump::NoTrump;
    /// use bridge_core::score::tables::{POINTS_OVER_TRICK};
    /// use karty::suits::SuitStd::Hearts;
    /// let contract = Contract::new(North, Bid::init(Trump::Colored(Hearts), 2).unwrap());
    /// let points_table = POINTS_OVER_TRICK;
    /// assert_eq!(points_table.points(&contract, 8 ,false), 0);
    /// assert_eq!(points_table.points(&contract, 10 ,false), 60);
    /// let contract = Contract::new_d(North, Bid::init(Trump::Colored(Hearts), 2).unwrap(), Double);
    /// assert_eq!(points_table.points(&contract, 7 ,false), 0);
    /// assert_eq!(points_table.points(&contract, 10 ,false), 200);
    /// assert_eq!(points_table.points(&contract, 11 ,true), 600);
    /// let contract = Contract::new_d(North, Bid::init(Trump::Colored(Hearts), 2).unwrap(), ReDouble);
    /// assert_eq!(points_table.points(&contract, 12 ,true), 1600);
    ///
    /// ```
    pub fn points(&self, contract: &Contract<SuitStd>, taken: u8, vulnerable: bool) -> i32 {

        let number_of_overtricks = taken.checked_sub(contract.bid().number_normalised()).unwrap_or(0);
        (number_of_overtricks as i32) * match contract.doubling() {
            Doubling::None => match contract.bid().trump() {
                Trump::Colored(SuitStd::Clubs) => self.not_doubled_clubs,
                Trump::Colored(SuitStd::Diamonds) => self.not_doubled_diamonds,
                Trump::Colored(SuitStd::Hearts) => self.not_doubled_hearts,
                Trump::Colored(SuitStd::Spades) => self.not_doubled_spades,
                Trump::NoTrump => self.not_doubled_nt
            }
            Doubling::Double => match vulnerable {
                true => self.doubled_vulnerable,
                false => self.doubled_not_vulnerable
            }
            Doubling::ReDouble => match vulnerable {
                true => self.redoubled_vulnerable,
                false => self.redoubled_not_vulnerable
            }
        }
    }
}

pub const POINTS_OVER_TRICK: PointsOverTrick = PointsOverTrick{
    not_doubled_clubs: 20,
    not_doubled_diamonds: 20,
    not_doubled_hearts: 30,
    not_doubled_spades: 30,
    not_doubled_nt: 30,
    doubled_not_vulnerable: 100,
    doubled_vulnerable: 200,
    redoubled_not_vulnerable: 200,
    redoubled_vulnerable: 400
};