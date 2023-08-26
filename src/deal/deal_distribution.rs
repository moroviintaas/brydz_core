use std::boxed::Box;
use crate::deal::BiasedHandDistribution;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum DealDistribution{
    Fair,
    Biased(Box<BiasedHandDistribution>),
}