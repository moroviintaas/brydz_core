use karty::hand::CardSet;
use sztorm::state::agent::{ScoringInformationSet};
use crate::contract::Contract;
use crate::deal::BiasedHandDistribution;
use crate::player::side::Side;
use crate::sztorm::spec::ContractDP;

pub trait RenewableContractInfoSet: ScoringInformationSet<ContractDP>{
    fn renew(&mut self, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>);

}

pub trait CreatedContractInfoSet: ScoringInformationSet<ContractDP>{
    fn create_new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>, distribution: BiasedHandDistribution) -> Self;
}