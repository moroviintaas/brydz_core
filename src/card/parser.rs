use nom::bytes::complete::{tag_no_case};
use nom::{IResult};
use nom::branch::alt;
use nom::character::complete::{digit1, space0};
use nom::error::ErrorKind;
use nom::sequence::{delimited, separated_pair};
use crate::card::Card;
use crate::card::figure::{MAX_NUMBER_FIGURE, FigureStd, NumberFigureStd, MIN_NUMBER_FIGURE};
use crate::card::suit::SuitStd;



/// Parses Ace
/// ```
/// use bridge_core::card::figure::FigureStd;
/// use bridge_core::card::parser::parse_ace;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_ace("acedd"), Ok(("dd", FigureStd::Ace)));
/// assert_eq!(parse_ace("aCe dd"), Ok((" dd", FigureStd::Ace)));
/// assert_eq!(parse_ace("qd dd"), Err(nom::Err::Error(nom::error::Error::new("qd dd", ErrorKind::Tag))));
/// ```
pub fn parse_ace(s: &str) -> IResult<&str, FigureStd>{
    alt((tag_no_case("ace"), tag_no_case("a")))(s)
        .map(|(i, _)| (i, FigureStd::Ace))
}
pub fn parse_king(s: &str) -> IResult<&str, FigureStd>{
    alt((tag_no_case("king"), tag_no_case("k")))(s)
        .map(|(i, _)| (i, FigureStd::King))
}
pub fn parse_queen(s: &str) -> IResult<&str, FigureStd>{
    alt((tag_no_case("queen"), tag_no_case("q")))(s)
        .map(|(i, _)| (i, FigureStd::Queen))
}
pub fn parse_jack(s: &str) -> IResult<&str, FigureStd>{
    alt((tag_no_case("jack"), tag_no_case("j")))(s)
        .map(|(i, _)| (i, FigureStd::Jack))
}

/// Parser numbered figure
/// ```
/// use bridge_core::card::figure::{FigureStd, NumberFigureStd};
/// use bridge_core::card::parser::parse_numbered_figure;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_numbered_figure("10fg"), Ok(("fg", FigureStd::Numbered(NumberFigureStd::new(10)))));
/// assert_eq!(parse_numbered_figure("11fg"), Err(nom::Err::Error(nom::error::Error::new("11fg", ErrorKind::Digit))));
/// assert_eq!(parse_numbered_figure("512fg"), Err(nom::Err::Error(nom::error::Error::new("512fg", ErrorKind::Fail))));
/// ```
pub fn parse_numbered_figure(s: &str) -> IResult<&str, FigureStd>{
    match digit1(s){
        Ok((i, ns)) => match ns.parse::<u8>(){
            Ok(n @MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE )=> Ok((i, FigureStd::Numbered(NumberFigureStd::new(n)))),
            Ok(_) => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Digit))),

            Err(_) => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Fail)))
        }
        Err(e) => Err(e)
    }
    /*
    match map_res(digit1, |ns: &str| ns.parse::<u8>())(s){
        Ok((i, n@ MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE)) => Ok((i, Figure::Numbered(NumberFigure::new(n)))),
        Ok((_, _))  => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::TooLarge))),

        Err(e) => Err(e)
    }*/
}
/// Parses a figure
/// ```
/// use bridge_core::card::figure::{FigureStd, NumberFigureStd};
/// use bridge_core::card::parser::parse_figure;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_figure("kc"), Ok(("c", FigureStd::King)));
/// assert_eq!(parse_figure("qdiamonds"), Ok(("diamonds", FigureStd::Queen)));
/// assert_eq!(parse_figure("deadfish"), Err(nom::Err::Error(nom::error::Error::new("deadfish", ErrorKind::Tag))));
/// ```
pub fn parse_figure(s: &str) -> IResult<&str, FigureStd>{
    alt((parse_numbered_figure, parse_ace, parse_king, parse_queen, parse_jack))(s)
}

pub fn parse_spades(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("spades"), tag_no_case("s")))(s)
        .map(|(i,_) | (i, SuitStd::Spades))
}
pub fn parse_hearts(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("hearts"), tag_no_case("h")))(s)
        .map(|(i,_) | (i, SuitStd::Hearts))
}
pub fn parse_diamonds(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("diamonds"), tag_no_case("diax"), tag_no_case("d")))(s)
        .map(|(i,_) | (i, SuitStd::Diamonds))
}
pub fn parse_clubs(s: &str) -> IResult<&str, SuitStd>{
    alt((tag_no_case("clubs"), tag_no_case("c")))(s)
        .map(|(i,_) | (i, SuitStd::Clubs))
}

/// Parses a suit
/// ```
/// use bridge_core::card::suit::SuitStd;
/// use bridge_core::card::parser::{parse_figure, parse_suit};
/// use nom::error::ErrorKind;
/// assert_eq!(parse_suit("sgq"), Ok(("gq", SuitStd::Spades)));
/// assert_eq!(parse_suit("diamondsda"), Ok(("da", SuitStd::Diamonds)));
/// assert_eq!(parse_suit("eadfish"), Err(nom::Err::Error(nom::error::Error::new("eadfish", ErrorKind::Tag))));
/// ```
pub fn parse_suit(s: &str) -> IResult<&str, SuitStd>{
    alt((parse_spades, parse_hearts, parse_diamonds, parse_clubs))(s)
}



/// Parses card from &str (strict way)
/// ```
/// use bridge_core::card::Card;
/// use bridge_core::card::figure::{FigureStd, NumberFigureStd};
/// use bridge_core::card::parser::parse_card_fs;
/// use bridge_core::card::suit::SuitStd;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_card_fs("10 dxg"), Ok(("xg", Card::new(FigureStd::Numbered(NumberFigureStd::new(10)), SuitStd::Diamonds))));
/// assert_eq!(parse_card_fs("A  sdiax"), Ok(("diax", Card::new(FigureStd::Ace, SuitStd::Spades))));
/// assert_eq!(parse_card_fs("A10  sdiax"), Err(nom::Err::Error(nom::error::Error::new("10  sdiax", ErrorKind::Tag))));
/// ```
pub fn parse_card_fs(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>>{
    match separated_pair(parse_figure, space0, parse_suit)(s){
        Ok((i, (fig, suit))) => Ok((i, Card::new(fig, suit))),
        Err(e) => Err(e)
    }

}

/// Parses card from &str
/// ```
/// use bridge_core::card::Card;
/// use bridge_core::card::figure::{FigureStd, NumberFigureStd};
/// use bridge_core::card::parser::parse_card_fs_delimited;
/// use bridge_core::card::suit::SuitStd;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_card_fs_delimited("  10 d\txg"), Ok(("xg", Card::new(FigureStd::Numbered(NumberFigureStd::new(10)), SuitStd::Diamonds))));
/// assert_eq!(parse_card_fs_delimited(" A  s\tdiax  "), Ok(("diax  ", Card::new(FigureStd::Ace, SuitStd::Spades))));
/// assert_eq!(parse_card_fs_delimited("\tA10  sdiax  "), Err(nom::Err::Error(nom::error::Error::new("10  sdiax  ", ErrorKind::Tag))));
/// ```
pub fn parse_card_fs_delimited(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>> {
    delimited(space0, parse_card_fs, space0)(s)
}



pub fn parse_card_sf(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>> {
    match separated_pair(parse_suit, space0, parse_figure)(s) {
        Ok((i, (suit, figure))) => Ok((i, Card::new(figure, suit))),
        Err(e) => Err(e)
    }
}

pub fn parse_card_sf_delimited(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>> {
    delimited(space0, parse_card_sf, space0)(s)
}

/// Parses card from &str (non delimeted way)
/// ```
/// use bridge_core::card::Card;
/// use bridge_core::card::figure::{FigureStd, NumberFigureStd};
/// use bridge_core::card::parser::parse_card;
/// use bridge_core::card::suit::SuitStd;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_card("10 dxg"), Ok(("xg", Card::new(FigureStd::Numbered(NumberFigureStd::new(10)), SuitStd::Diamonds))));
/// assert_eq!(parse_card("A  sdiax"), Ok(("diax", Card::new(FigureStd::Ace, SuitStd::Spades))));
/// assert_eq!(parse_card("h  jv"), Ok(("v", Card::new(FigureStd::Jack, SuitStd::Hearts))));
/// assert_eq!(parse_card("A10  sdiax"), Err(nom::Err::Error(nom::error::Error::new("A10  sdiax", ErrorKind::Tag))));
/// ```
pub fn parse_card(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>> {
    alt((parse_card_fs, parse_card_sf))(s)
}

/// Parses card from &str (delimited way)
/// ```
/// use bridge_core::card::Card;
/// use bridge_core::card::figure::{FigureStd, NumberFigureStd};
/// use bridge_core::card::parser::parse_card_delimited;
/// use bridge_core::card::suit::SuitStd;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_card_delimited("10 d  xg"), Ok(("xg", Card::new(FigureStd::Numbered(NumberFigureStd::new(10)), SuitStd::Diamonds))));
/// assert_eq!(parse_card_delimited("A  s\tdiax"), Ok(("diax", Card::new(FigureStd::Ace, SuitStd::Spades))));
/// assert_eq!(parse_card_delimited("   h  jv"), Ok(("v", Card::new(FigureStd::Jack, SuitStd::Hearts))));
/// assert_eq!(parse_card_delimited(" A10  sdiax"), Err(nom::Err::Error(nom::error::Error::new("A10  sdiax", ErrorKind::Tag))));
/// ```
pub fn parse_card_delimited(s: &str) -> IResult<&str, Card<FigureStd, SuitStd>>{
    delimited(space0, parse_card, space0)(s)
}
