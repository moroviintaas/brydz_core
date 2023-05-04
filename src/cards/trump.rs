use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash};
use rand::Rng;
use karty::random::RandomSymbol;
use karty::suits::{SuitTrait, Suit};
use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};

#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

use crate::cards::trump::TrumpGen::{Colored, NoTrump};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
pub enum TrumpGen<S: SuitTrait>{
    Colored(S),
    NoTrump
}

impl<R: Rng, S: SuitTrait> RandomSymbol<R> for TrumpGen<S>{
    fn random(rng: &mut R) -> Self {
        match rng.gen_range(0..=S::SYMBOL_SPACE){
            special if special == S::SYMBOL_SPACE => NoTrump,
            i => Colored(S::from_position(i).unwrap())
        }
    }
}
/*

impl<S: SuitTrait> Distribution<TrumpGen<S>> for Standard
where Standard: Distribution<S>{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TrumpGen<S> {
        match rng.gen_range(0..=S::SYMBOL_SPACE){
            special if special == S::SYMBOL_SPACE => NoTrump,
            i => Colored(S::from_position(i).unwrap())
        }
    }
}*/



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

#[cfg(feature = "serde_dedicate")]
mod serialize_dedicated{
    use crate::cards::trump::TrumpGen::NoTrump;
    use crate::cards::trump::Suit::*;
    use crate::cards::trump::TrumpGen::*;
    use crate::cards::trump::{Suit, Trump};
    use crate::cards::trump::TrumpGen;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Deserialize, Serialize)]
    enum FlatTrump{NoTrump, Spades, Hearts, Diamonds, Clubs}

    impl From<FlatTrump> for Trump{
        fn from(value: FlatTrump) -> Self {
            match value{
                FlatTrump::NoTrump => NoTrump,
                FlatTrump::Spades => Colored(Spades),
                FlatTrump::Hearts => Colored(Hearts),
                FlatTrump::Diamonds => Colored(Diamonds),
                FlatTrump::Clubs => Colored(Clubs)
            }
        }
    }

    impl From<&Trump> for FlatTrump{
        fn from(value: &Trump) -> Self {
            match value{
                Colored(s) => match s{
                    Spades => FlatTrump::Spades,
                    Hearts => FlatTrump::Hearts,
                    Diamonds => FlatTrump::Diamonds,
                    Clubs => FlatTrump::Clubs
                }
                NoTrump => FlatTrump::NoTrump
            }
        }
    }



    impl Serialize for TrumpGen<Suit>{
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {


            match FlatTrump::from(self){
                FlatTrump::NoTrump => serializer.serialize_unit_variant("Trump", 4, "NoTrump"),
                FlatTrump::Spades => serializer.serialize_unit_variant("Trump", 3, "Spades"),
                FlatTrump::Hearts => serializer.serialize_unit_variant("Trump", 2, "Hearts"),
                FlatTrump::Diamonds => serializer.serialize_unit_variant("Trump", 1, "Diamonds"),
                FlatTrump::Clubs => serializer.serialize_unit_variant("Trump", 0, "Clubs")
            }


        }
    }


    impl<'de> Deserialize<'de> for TrumpGen<Suit>{
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            FlatTrump::deserialize(deserializer).map(|ft| ft.into())
        }
    }


}


#[cfg(test)]
mod tests{
    use karty::suits::Suit;
    use karty::suits::Suit::*;
    use crate::cards::trump::TrumpGen;

    #[test]
    #[cfg(feature = "serde_dedicate")]
    fn serialize_trump(){
        use ron;

        let hearts = TrumpGen::Colored(Hearts);
        assert_eq!(ron::to_string(&hearts).unwrap(), "Hearts");
        assert_eq!(ron::to_string(&TrumpGen::<Suit>::NoTrump).unwrap(), "NoTrump");
    }

    #[test]
    #[cfg(feature = "serde_dedicate")]
    fn deserialize_trump(){
        use ron;
        assert_eq!(ron::from_str::<TrumpGen<Suit>>("NoTrump").unwrap(), TrumpGen::NoTrump);
        assert_eq!(ron::from_str::<TrumpGen<Suit>>("Diamonds").unwrap(), TrumpGen::Colored(Diamonds));
    }

    #[test]
    #[cfg(feature = "serde_derive")]
    fn serialize_trump_derive(){
        use ron;

        let hearts = TrumpGen::Colored(Hearts);
        assert_eq!(ron::to_string(&hearts).unwrap(), "Colored(Hearts)");
        assert_eq!(ron::to_string(&TrumpGen::<Suit>::NoTrump).unwrap(), "NoTrump");
    }
}

