use nom::branch::alt;
use nom::bytes::complete::{tag_no_case};
use nom::character::complete::{digit1, space0};
use nom::IResult;
use nom::sequence::{delimited, separated_pair};
use crate::auction::bid::Bid;
use crate::play::trump::Trump;
use nom::error::ErrorKind;
use karty::suits::parse::parse_suit;
use karty::suits::SuitStd;


///Parses no trump strict
///```
/// use bridge_core::auction::parser::parse_nt;
/// use bridge_core::play::trump::Trump;
/// assert_eq!(parse_nt("nt "), Ok((" ", Trump::NoTrump)));
/// assert_eq!(parse_nt("notrump"), Ok(("", Trump::NoTrump)));
/// assert_eq!(parse_nt("n"), Ok(("", Trump::NoTrump)));
/// ```
pub fn parse_nt(s: &str) -> IResult<&str, Trump<SuitStd>>{
    alt((tag_no_case("no_trump"), tag_no_case("notrump"), tag_no_case("nt"), tag_no_case("n")))(s)
        .map(|(i,_)| (i, Trump::NoTrump ))
}
///Parses no trump (delimited)
///```
/// use bridge_core::auction::parser::parse_nt_delimited;
/// use bridge_core::play::trump::Trump;
/// assert_eq!(parse_nt_delimited("\tnt \t"), Ok(("", Trump::NoTrump)));
/// assert_eq!(parse_nt_delimited("  notrump\t"), Ok(("", Trump::NoTrump)));
/// assert_eq!(parse_nt_delimited("  n "), Ok(("", Trump::NoTrump)));
/// ```
pub fn parse_nt_delimited(s: &str) -> IResult<&str, Trump<SuitStd>>{
    delimited(space0, parse_nt, space0)(s)
}

/// Parses colored trump
/// ```
/// use bridge_core::auction::parser::parse_trump_colored;
/// use karty::suits::standard::SuitStd::{Spades, Hearts};
/// use bridge_core::play::trump::Trump;
/// assert_eq!(parse_trump_colored("hjik"), Ok(("jik", Trump::Colored(Hearts))));
/// assert_eq!(parse_trump_colored("spadesorsth"), Ok(("orsth", Trump::Colored(Spades))));
/// ```
pub fn parse_trump_colored(s: &str) -> IResult<&str, Trump<SuitStd>>{
    parse_suit(s).map(|(r, s)| (r, Trump::Colored(s)))
}

/// Parses trump
/// ```
/// use bridge_core::auction::parser::{parse_nt, parse_trump_colored};
/// use karty::suits::standard::SuitStd::{Spades, Hearts};
/// use bridge_core::play::trump::Trump;
/// assert_eq!(parse_trump_colored("hjik"), Ok(("jik", Trump::Colored(Hearts))));
/// assert_eq!(parse_trump_colored("spadesorsth"), Ok(("orsth", Trump::Colored(Spades))));
/// assert_eq!(parse_nt("notrump\t"), Ok(("\t", Trump::NoTrump)));
/// ```
pub fn parse_trump(s: &str) -> IResult<&str, Trump<SuitStd>>{
    alt((parse_trump_colored, parse_nt))(s)
}
/// parses bid
/// ```
/// use bridge_core::auction::parser::parse_bid;
/// use bridge_core::auction::bid::Bid;
/// use karty::suits::SuitStd::Clubs;
/// use bridge_core::play::trump::Trump;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_bid("3c"), Ok(("", Bid::create_bid(Trump::Colored(Clubs), 3).unwrap())));
/// assert_eq!(parse_bid("7nt "), Ok((" ", Bid::create_bid(Trump::NoTrump, 7).unwrap())));
/// assert_eq!(parse_bid("0hearts"), Err(nom::Err::Error(nom::error::Error::new("0hearts", ErrorKind::Digit))));
/// assert_eq!(parse_bid("q2spades"), Err(nom::Err::Error(nom::error::Error::new("q2spades", ErrorKind::Digit))));
/// assert_eq!(parse_bid("8spades"), Err(nom::Err::Error(nom::error::Error::new("8spades", ErrorKind::Digit))));
/// assert_eq!(parse_bid("h"), Err(nom::Err::Error(nom::error::Error::new("h", ErrorKind::Digit))));
/// ```
pub fn parse_bid(s: &str) -> IResult<&str, Bid<SuitStd>>{
    match separated_pair(digit1, space0, parse_trump)(s){
        Ok((remains, (digs, trump))) => match digs.parse::<u8>(){
            Ok(n) => Bid::create_bid(trump, n).map_or_else(
                |_| Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Digit))),
                | bid|  Ok((remains, bid))),
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Digit)))
        },
        Err(e) => Err(e)
    }
        //.map(|(i, (digs, trump))| )
}
