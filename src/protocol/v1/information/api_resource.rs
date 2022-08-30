use karty::cards::CardStd;
use crate::player::axis::Axis;
use crate::player::side::Side;

pub enum ApiResource {
    Side(Side),
    Axis(Axis),
    Card(CardStd),
    CardVec(Vec<CardStd>)

}