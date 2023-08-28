use std::boxed::Box;
use rand::distributions::Distribution;
use rand::Rng;
use rand::seq::SliceRandom;
use karty::cards::STANDARD_DECK;
use karty::hand::CardSet;
use crate::deal::{BiasedHandDistribution, distribute_standard_deck_on_4, fair_bridge_deal};
use crate::player::side::SideMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum DealDistribution{
    Fair,
    Biased(Box<BiasedHandDistribution>),
}

impl Distribution<SideMap<CardSet>> for DealDistribution{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SideMap<CardSet> {
        match self{
            DealDistribution::Biased(distr) => {
                distr.as_ref().sample(rng)
            },
            DealDistribution::Fair => {
                distribute_standard_deck_on_4(rng)

            }
        }
    }
}