use karty::suits::SuitStd;
use crate::bidding::Doubling;
use crate::deal::Contract;
use crate::cards::trump::Trump;

pub struct PointsContractedTrick{
    pub clubs: i32,
    pub diamonds: i32,
    pub hearts: i32,
    pub spades: i32,
    pub nt_first: i32,
    pub nt_next: i32,
    pub doubling_multiplier: i32,
    pub redoubling_multiplier: i32

}
impl PointsContractedTrick{
    /// Calculates points for contracted tricks based on number of taken, does not count overtricks
    /// # Examples:
    /// ```
    /// use bridge_core::deal::Contract;
    /// use bridge_core::player::side::Side::North;
    /// use bridge_core::bidding::Bid;
    /// use bridge_core::bidding::Doubling::ReDouble;
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::cards::trump::Trump::NoTrump;
    /// use bridge_core::score::tables::POINTS_CONTRACTED_TRICK;
    /// use karty::suits::SuitStd::Hearts;
    /// let contract = Contract::new(North, Bid::init(Trump::Colored(Hearts), 2).unwrap());
    /// let points_table = POINTS_CONTRACTED_TRICK;
    /// assert_eq!(points_table.points(&contract, 7), 210 );
    /// assert_eq!(points_table.points(&contract, 8), 240 );
    /// assert_eq!(points_table.points(&contract, 9), 240 );
    ///
    /// let contract = Contract::new_d(North, Bid::init(NoTrump, 1).unwrap(), ReDouble);
    /// assert_eq!(points_table.points(&contract, 6), 760 );
    /// assert_eq!(points_table.points(&contract, 7), 880 );
    /// assert_eq!(points_table.points(&contract, 8), 880 );
    ///
    /// ```

    pub fn points(&self, contract: &Contract<SuitStd>, taken: u8) -> i32{
        let multiplier = match contract.doubling(){
            Doubling::None => 1,
            Doubling::Double => self.doubling_multiplier,
            Doubling::ReDouble => self.redoubling_multiplier,
        };
        match contract.bid().trump(){
            Trump::Colored(c) => {

                let number = if contract.bid().number_normalised() < taken{
                    contract.bid().number_normalised()
                } else{
                    taken
                };
                i32::from(number) * multiplier * match c{
                    SuitStd::Spades => &self.spades,
                    SuitStd::Hearts => &self.hearts,
                    SuitStd::Diamonds => &self.diamonds,
                    SuitStd::Clubs => &self.clubs
                }
            }
            Trump::NoTrump => {
                if taken == 0{
                    i32::from(0)
                } else{
                    let number = if contract.bid().number_normalised() < taken{
                        contract.bid().number_normalised() - 1
                    }
                    else{
                        taken - 1
                    };
                    (&self.nt_first + (&self.nt_next * i32::from(number))) * multiplier

                }

            }
        }
    }
}

pub const POINTS_CONTRACTED_TRICK: PointsContractedTrick = PointsContractedTrick{
    clubs: 20,
    diamonds: 20,
    hearts: 30,
    spades: 30,
    nt_first: 40,
    nt_next: 30,
    doubling_multiplier: 2,
    redoubling_multiplier: 4
};