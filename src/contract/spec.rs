
//use serde::{Deserialize, Serialize};
use karty::suits::SuitTrait;
use crate::error::BiddingErrorGen::{DoubleAfterDouble, DoubleAfterReDouble, ReDoubleAfterReDouble, ReDoubleWithoutDouble};
use crate::bidding::{Doubling};
use crate::player::side::Side;
use crate::bidding::Bid;
use crate::error::BiddingErrorGen;


#[derive(Debug, Eq, PartialEq,  Clone)]
//#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContractSpec<S: SuitTrait> {
    declarer: Side,
    bid: Bid<S>,
    doubling: Doubling
}

impl<S: SuitTrait> ContractSpec<S> {
    pub fn new_d(owner: Side, bid: Bid<S>, doubling: Doubling) -> Self{
        Self{bid, doubling, declarer: owner }
    }
    pub fn new(player: Side, bid: Bid<S>) -> Self{
        Self{ declarer: player, bid, doubling: Doubling::None}
    }
    pub fn bid(&self) -> &Bid<S>{
        &self.bid
    }
    pub fn doubling(&self) -> Doubling{
        self.doubling
    }
    pub fn declarer(&self) -> Side{
        self.declarer
    }

    pub fn double(&mut self) -> Result<(), BiddingErrorGen<S>>{
        match self.doubling{
            Doubling::None => {
                self.doubling = Doubling::Double;
                Ok(())
            },
            Doubling::Double => Err(DoubleAfterDouble),
            Doubling::ReDouble => Err(DoubleAfterReDouble)
        }
    }

    pub fn redouble(&mut self) -> Result<(), BiddingErrorGen<S>>{
        match self.doubling{
            Doubling::Double => {
                self.doubling = Doubling::ReDouble;
                Ok(())
            },
            Doubling::ReDouble => Err(ReDoubleAfterReDouble),
            Doubling::None => Err(ReDoubleWithoutDouble)
        }
    }

}

#[cfg(feature = "serde")]
mod serde_for_contract_spec{
    use std::fmt::Formatter;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
    use serde::de::{MapAccess, SeqAccess, Visitor};
    use serde::ser::SerializeStruct;
    use karty::suits::Suit;
    use crate::contract::ContractSpec;
    use crate::player::side::Side;

    impl Serialize for ContractSpec<Suit>{
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
            let mut state = serializer.serialize_struct("contract", 3)?;
            state.serialize_field("declarer", &self.declarer)?;
            state.serialize_field("bid", &self.bid)?;
            state.serialize_field("doubling", &self.doubling)?;
            state.end()
        }
    }

    impl<'de> Deserialize<'de> for ContractSpec<Suit>{
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            #[derive(Deserialize)]
            #[serde(field_identifier, rename_all = "lowercase")]
            enum Field { Declarer, Bid, Doubling }
            struct ContractSpecVisitor;
            impl<'de> Visitor<'de> for ContractSpecVisitor{
                type Value = ContractSpec<Suit>;

                fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                    formatter.write_str("Expected struct with fields [Declarer(Side), Bid(Bid<Suit>), Doubling]")
                }
                fn visit_seq<V>(self, mut seq: V) -> Result<ContractSpec<Suit>, V::Error>
                where V: SeqAccess<'de> {
                    let declarer = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let bid = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let doubling = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                    Ok(ContractSpec::<Suit>::new_d(declarer, bid, doubling))

                }

                fn visit_map<V>(self, mut map: V) -> Result<ContractSpec<Suit>, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut declarer_op = None;
                    let mut bid_op = None;
                    let mut doubling_op = None;

                    while let Some(key) = map.next_key()?{
                        match key {
                            Field::Declarer => {
                                if declarer_op.is_some(){
                                    return Err(de::Error::duplicate_field("declarer"));
                                }
                                declarer_op = Some(map.next_value()?);
                            }
                            Field::Bid => {
                                if bid_op.is_some(){
                                    return Err(de::Error::duplicate_field("bid"));
                                }
                                bid_op = Some(map.next_value()?);
                            }
                            Field::Doubling => {
                                if doubling_op.is_some(){
                                    return Err(de::Error::duplicate_field("doubling"));
                                }
                                doubling_op = Some(map.next_value()?);
                            }
                        }
                    }
                    let declarer = declarer_op.ok_or_else(|| de::Error::missing_field("declarer"))?;
                    let bid = bid_op.ok_or_else(|| de::Error::missing_field("bid"))?;
                    let doubling = doubling_op.ok_or_else(|| de::Error::missing_field("doubling"))?;
                    Ok(ContractSpec::<Suit>::new_d(declarer, bid, doubling))
                }
            }
            const FIELDS: &'static [&'static str] = &["declarer", "bid", "doubling"];
            deserializer.deserialize_struct("contract", FIELDS, ContractSpecVisitor)
        }
    }
}

#[cfg(test)]
mod tests{
    use karty::suits::Suit;
    use karty::suits::Suit::{Diamonds, Hearts};
    use crate::bidding::Bid;
    use crate::bidding::Doubling::{Double, ReDouble};
    use crate::cards::trump::TrumpGen;
    use crate::contract::ContractSpec;
    use crate::player::side::Side::*;

    #[test]
    #[cfg(feature = "serde")]
    fn serialize_contract_spec(){
        let contract_1 = ContractSpec::new_d(
            East,
            Bid::init(TrumpGen::Colored(Diamonds), 4).unwrap(),
            ReDouble
        );
        assert_eq!(ron::to_string(&contract_1).unwrap(), "(declarer:East,bid:(trump:\"Diamonds\",number:4),doubling:ReDouble)");

    }

    #[test]
    #[cfg(feature = "serde")]
    fn deserialize_contract_spec(){
        let contract_1 = ContractSpec::new_d(
            West,
            Bid::init(TrumpGen::NoTrump, 6).unwrap(),
            Double
        );
        assert_eq!(ron::from_str::<ContractSpec<Suit>>("(declarer:West, doubling:Double, bid: (trump:\"NoTrump\",number:6))").unwrap(), contract_1);

    }
}