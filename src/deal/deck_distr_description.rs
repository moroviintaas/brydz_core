use karty::hand::CardSet;
use crate::deal::{BiasedHandDistribution, DealDistribution};
use crate::player::side::SideMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DescriptionDeckDeal{
    pub probabilities: DealDistribution,
    pub cards: SideMap<CardSet>
}
