use nom::bytes::complete::{tag, tag_no_case};
use nom::{IResult};
use nom::branch::alt;
use nom::character::complete::digit1;
use nom::error::ErrorKind;
use nom::multi::{many0};
use nom::sequence::{separated_pair};
use crate::cards::Card;
use crate::cards::figure::{MAX_NUMBER_FIGURE, Figure, NumberFigure, MIN_NUMBER_FIGURE};
use crate::cards::suit::Suit;



/// Parses Ace
/// ```
/// use bridge_core::cards::figure::Figure;
/// use bridge_core::cards::parser::parse_ace;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_ace("acedd"), Ok(("dd", Figure::Ace)));
/// assert_eq!(parse_ace("aCe dd"), Ok((" dd", Figure::Ace)));
/// assert_eq!(parse_ace("qd dd"), Err(nom::Err::Error(nom::error::Error::new("qd dd", ErrorKind::Tag))));
/// ```
pub fn parse_ace(s: &str) -> IResult<&str, Figure>{
    alt((tag_no_case("ace"), tag_no_case("a")))(s)
        .map(|(i, _)| (i, Figure::Ace))
}
pub fn parse_king(s: &str) -> IResult<&str, Figure>{
    alt((tag_no_case("king"), tag_no_case("k")))(s)
        .map(|(i, _)| (i, Figure::King))
}
pub fn parse_queen(s: &str) -> IResult<&str, Figure>{
    alt((tag_no_case("queen"), tag_no_case("q")))(s)
        .map(|(i, _)| (i, Figure::Queen))
}
pub fn parse_jack(s: &str) -> IResult<&str, Figure>{
    alt((tag_no_case("jack"), tag_no_case("j")))(s)
        .map(|(i, _)| (i, Figure::Jack))
}

/// Parser numbered figure
/// ```
/// use bridge_core::cards::figure::{Figure, NumberFigure};
/// use bridge_core::cards::parser::parse_numbered_figure;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_numbered_figure("10fg"), Ok(("fg", Figure::Numbered(NumberFigure::new(10)))));
/// assert_eq!(parse_numbered_figure("11fg"), Err(nom::Err::Error(nom::error::Error::new("11fg", ErrorKind::TooLarge))));
/// assert_eq!(parse_numbered_figure("512fg"), Err(nom::Err::Error(nom::error::Error::new("512fg", ErrorKind::Fail))));
/// ```
pub fn parse_numbered_figure(s: &str) -> IResult<&str, Figure>{
    match digit1(s){
        Ok((i, ns)) => match ns.parse::<u8>(){
            Ok(n @MIN_NUMBER_FIGURE..=MAX_NUMBER_FIGURE )=> Ok((i, Figure::Numbered(NumberFigure::new(n)))),
            Ok(_) => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::TooLarge))),
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
/// use bridge_core::cards::figure::{Figure, NumberFigure};
/// use bridge_core::cards::parser::parse_figure;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_figure("kc"), Ok(("c", Figure::King)));
/// assert_eq!(parse_figure("qdiamonds"), Ok(("diamonds", Figure::Queen)));
/// assert_eq!(parse_figure("deadfish"), Err(nom::Err::Error(nom::error::Error::new("deadfish", ErrorKind::Tag))));
/// ```
pub fn parse_figure(s: &str) -> IResult<&str, Figure>{
    alt(( parse_numbered_figure, parse_ace, parse_king, parse_queen, parse_jack))(s)
}

pub fn parse_spades(s: &str) -> IResult<&str, Suit>{
    alt((tag_no_case("spades"), tag_no_case("s")))(s)
        .map(|(i,_) | (i, Suit::Spades))
}
pub fn parse_hearts(s: &str) -> IResult<&str, Suit>{
    alt((tag_no_case("hearts"), tag_no_case("h")))(s)
        .map(|(i,_) | (i, Suit::Hearts))
}
pub fn parse_diamonds(s: &str) -> IResult<&str, Suit>{
    alt((tag_no_case("diamonds"), tag_no_case("diax"), tag_no_case("d")))(s)
        .map(|(i,_) | (i, Suit::Diamonds))
}
pub fn parse_clubs(s: &str) -> IResult<&str, Suit>{
    alt((tag_no_case("clubs"), tag_no_case("c")))(s)
        .map(|(i,_) | (i, Suit::Clubs))
}

/// Parses a suit
/// ```
/// use bridge_core::cards::suit::Suit;
/// use bridge_core::cards::parser::{parse_figure, parse_suit};
/// use nom::error::ErrorKind;
/// assert_eq!(parse_suit("sgq"), Ok(("gq", Suit::Spades)));
/// assert_eq!(parse_suit("diamondsda"), Ok(("da", Suit::Diamonds)));
/// assert_eq!(parse_suit("eadfish"), Err(nom::Err::Error(nom::error::Error::new("eadfish", ErrorKind::Tag))));
/// ```
pub fn parse_suit(s: &str) -> IResult<&str, Suit>{
    alt((parse_spades, parse_hearts, parse_diamonds, parse_clubs))(s)
}



/// Parses card from &str
/// ```
/// use bridge_core::cards::Card;
/// use bridge_core::cards::figure::{Figure, NumberFigure};
/// use bridge_core::cards::parser::parse_card_fs;
/// use bridge_core::cards::suit::Suit;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_card_fs("10 dxg"), Ok(("xg", Card::new(Figure::Numbered(NumberFigure::new(10)), Suit::Diamonds))));
/// assert_eq!(parse_card_fs("A  sdiax"), Ok(("diax", Card::new(Figure::Ace, Suit::Spades))));
/// assert_eq!(parse_card_fs("A10  sdiax"), Err(nom::Err::Error(nom::error::Error::new("10  sdiax", ErrorKind::Tag))));
/// ```
pub fn parse_card_fs(s: &str) -> IResult<&str, Card>{
    match separated_pair(parse_figure, many0(tag(" ")), parse_suit)(s){
        Ok((i, (fig, suit))) => Ok((i, Card::new(fig, suit))),
        Err(e) => Err(e)
    }

}

pub fn parse_card_sf(s: &str) -> IResult<&str, Card> {
    match separated_pair(parse_suit, many0(tag(" ")), parse_figure)(s) {
        Ok((i, (suit, figure))) => Ok((i, Card::new(figure, suit))),
        Err(e) => Err(e)
    }
}

/// Parses card from &str
/// ```
/// use bridge_core::cards::Card;
/// use bridge_core::cards::figure::{Figure, NumberFigure};
/// use bridge_core::cards::parser::parse_card;
/// use bridge_core::cards::suit::Suit;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_card("10 dxg"), Ok(("xg", Card::new(Figure::Numbered(NumberFigure::new(10)), Suit::Diamonds))));
/// assert_eq!(parse_card("A  sdiax"), Ok(("diax", Card::new(Figure::Ace, Suit::Spades))));
/// assert_eq!(parse_card("h  jv"), Ok(("v", Card::new(Figure::Jack, Suit::Hearts))));
/// assert_eq!(parse_card("A10  sdiax"), Err(nom::Err::Error(nom::error::Error::new("A10  sdiax", ErrorKind::Tag))));
/// ```
pub fn parse_card(s: &str) -> IResult<&str, Card> {
    alt((parse_card_fs, parse_card_sf))(s)
}
