use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash};
use karty::suits::{SuitTrait, Suit};
use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
#[cfg(feature = "serde")]
use serde::{Serializer, Deserializer, Serialize, Deserialize};
#[cfg(feature = "serde")]
use serde::de::{Error, Visitor};

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use crate::cards::trump::TrumpGen::{Colored, NoTrump};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
//#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TrumpGen<S: SuitTrait>{
    Colored(S),
    NoTrump
}



pub type Trump = TrumpGen<Suit>;




impl<S: SuitTrait> PartialOrd for TrumpGen<S>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: SuitTrait> Ord for TrumpGen<S>{
    fn cmp(&self, other: &Self) -> Ordering {
        match self{
            NoTrump => match other{
                NoTrump => Ordering::Equal,
                _ => Ordering::Greater
            },
            Colored(left) => match other {
                NoTrump => Ordering::Less,
                Colored(right) => left.cmp(right)
            }
        }
    }
}

impl <S: SuitTrait + Display> Display for TrumpGen<S>{
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub const TRUMPS: [TrumpGen<Suit>; 5] = [Colored(Spades), Colored(Hearts), Colored(Diamonds), Colored(Clubs), NoTrump];
/*
#[cfg(feature = "serde")]
impl<ST: SuitTrait + Serialize> Serialize for TrumpGen<ST>{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {


        match self{
            Colored(c) => serializer.serialize_newtype_variant("Trump", 0, "Colored", c),
            NoTrump => serializer.serialize_unit_variant("Trump", 1, "NoTrump")
        }

    }
}*/

#[cfg(feature = "serde")]
impl Serialize for TrumpGen<Suit>{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {


        serializer.serialize_str(match self{
            Colored(s) => match s{
                Spades => "Spades",
                Hearts => "Hearts",
                Diamonds => "Diamonds",
                Clubs => "Clubs,"
            }
            NoTrump => "NoTrump"
        })

    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TrumpGen<Suit>{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(TrumpVisitor)
    }
}
#[cfg(feature = "serde")]
struct TrumpVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for TrumpVisitor{
    type Value = TrumpGen<Suit>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Spades/Hearts/Diamonds/Clubs/NoTrump")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        match &v.to_lowercase()[..]{
            "s" | "spades" => Ok(Colored(Spades)),
            "h" | "hearts" => Ok(Colored(Hearts)),
            "d" | "diamonds" => Ok(Colored(Diamonds)),
            "c" | "clubs" => Ok(Colored(Clubs)),
            "nt" | "notrump" | "no_trump" | "n" => Ok(NoTrump),
            unrecognised => Err(E::custom(format!("Unrecognised {unrecognised:}")))
        }
    }
}


#[cfg(test)]
mod tests{
    use karty::suits::Suit;
    use karty::suits::Suit::{*};
    use crate::cards::trump::TrumpGen;


    #[test]
    #[cfg(feature = "serde")]
    fn serialize_trump(){
        use ron;

        let hearts = TrumpGen::Colored(Hearts);
        assert_eq!(ron::to_string(&hearts).unwrap(), "\"Hearts\"");
        assert_eq!(ron::to_string(&TrumpGen::<Suit>::NoTrump).unwrap(), "\"NoTrump\"");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn deserialize_trump(){
        use ron;
        assert_eq!(ron::from_str::<TrumpGen<Suit>>("\"NoTrump\"").unwrap(), TrumpGen::NoTrump);
        assert_eq!(ron::from_str::<TrumpGen<Suit>>("\"Diamonds\"").unwrap(), TrumpGen::Colored(Diamonds));
    }
}

