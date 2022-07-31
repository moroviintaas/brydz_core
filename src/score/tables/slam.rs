use karty::suits::{SuitStd};
use crate::bidding::contract::Contract;
use crate::meta::QUARTER_SIZE;

pub struct PointsSlam{
    vulnerable_great_slam: i32,
    vulnerable_small_slam: i32,
    not_vulnerable_great_slam: i32,
    not_vulnerable_small_slam: i32,
}

pub const POINTS_SLAM: PointsSlam = PointsSlam{
    vulnerable_great_slam: 1500,
    vulnerable_small_slam: 750,
    not_vulnerable_great_slam: 1000,
    not_vulnerable_small_slam: 500
};

impl PointsSlam{
    ///
    /// # Args:
    /// `taken: u8` - number of tricks taken (in total)
    /// # Examples:
    /// ```
    /// use bridge_core::bidding::contract::Contract;
    /// use bridge_core::player::side::Side::North;
    /// use bridge_core::bidding::bid::Bid;
    /// use bridge_core::bidding::call::Doubling::ReDouble;
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::cards::trump::Trump::NoTrump;
    /// use bridge_core::score::tables::POINTS_SLAM;
    /// use karty::suits::SuitStd::Hearts;
    /// let contract = Contract::new(North, Bid::init(Trump::Colored(Hearts), 2).unwrap());
    /// let points_table = POINTS_SLAM;
    /// assert_eq!(points_table.points(&contract, 13, false), 0);
    /// let contract = Contract::new(North, Bid::init(Trump::Colored(Hearts), 6).unwrap());
    /// assert_eq!(points_table.points(&contract, 12, false), 500);
    /// assert_eq!(points_table.points(&contract, 12, true), 750);
    /// assert_eq!(points_table.points(&contract, 13, true), 750);
    /// let contract = Contract::new_d(North, Bid::init(NoTrump, 7).unwrap(), ReDouble);
    /// assert_eq!(points_table.points(&contract, 12, false), 0 );
    /// assert_eq!(points_table.points(&contract, 13, true), 1500 );
    /// assert_eq!(points_table.points(&contract, 13, false), 1000 );
    ///
    /// ```
    pub fn points(&self, contract: &Contract<SuitStd>, taken: u8, vulnerable: bool) -> i32{
        let declared = contract.bid().number_normalised() as usize;
        match declared{
            n if n == QUARTER_SIZE => {
                if taken >= declared as u8{
                    match vulnerable{
                        true => self.vulnerable_great_slam,
                        false => self.not_vulnerable_great_slam
                    }
                }
                else{
                    0
                }
            },
            n1 if n1 == (QUARTER_SIZE - 1) => {
                if taken >= declared as u8{
                    match vulnerable{
                        true => self.vulnerable_small_slam,
                        false => self.not_vulnerable_small_slam
                    }
                }
                else{
                    0
                }
            }
            _ => {0}
        }
    }
}
/*
impl<Co: ContractOverseer<F, S>, F:Figure, S: Suit> ScoreIngredient<Co, F, S> for PointsSlam{
    ///
    /// # Examples:
    /// ```
    /// use bridge_core::bidding::contract::Contract;
    /// use bridge_core::player::side::Side::North;
    /// use bridge_core::bidding::bid::Bid;
    /// use bridge_core::bidding::call::Doubling::ReDouble;
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::cards::trump::Trump::NoTrump;
    /// use bridge_core::score::tables::POINTS_SLAM;
    /// use karty::suits::SuitStd::Hearts;
    /// let contract = Contract::new(North, Bid::init(Trump::Colored(Hearts), 2).unwrap());
    /// let points_table = POINTS_SLAM;
    /// assert_eq!(points_table.points(&contract, 7), 210 );
    /// let contract = Contract::new_d(North, Bid::init(NoTrump, 1).unwrap(), ReDouble);
    /// assert_eq!(points_table.points(&contract, 6), 760 );
    /// assert_eq!(points_table.points(&contract, 7), 880 );
    /// assert_eq!(points_table.points(&contract, 8), 880 );
    ///
    /// ```
    fn calculate(&self, contract_overseer: Co, vulnerability: bool) -> Result<i32, BridgeError<F, S>> {
        if !contract_overseer.is_completed(){
            return Err(BridgeError::DealError(DealError::DealIncomplete));
        }
        let declared = contract_overseer.contract().bid().number_normalised() as usize;
        let scored = contract_overseer.total_tricks_taken_axis(contract_overseer.contract().declarer().axis());


        Ok(match declared as usize{
            n if n == QUARTER_SIZE => {
                if scored == declared{
                    match vulnerability{
                        true => self.vulnerable_great_slam,
                        false => self.not_vulnerable_great_slam
                    }
                }
                else{
                    0
                }
            },
            n1 if n1 == (QUARTER_SIZE - 1) => {
                if scored >= declared{
                    match vulnerability{
                        true => self.vulnerable_small_slam,
                        false => self.not_vulnerable_small_slam
                    }
                }
                else{
                    0
                }
            }
            _ => {0}
        })
    }

}
*/