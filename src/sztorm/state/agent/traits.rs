use karty::hand::CardSet;
use sztorm::state::agent::{ScoringInformationSet};
use crate::contract::{Contract, ContractMechanics};
use crate::deal::BiasedHandDistribution;
use crate::player::side::Side;
use crate::sztorm::spec::ContractDP;

pub trait ContractInfoSet{
    fn side(&self) -> Side;
    fn contract_data(&self) -> &Contract;
    fn dummy_hand(&self) -> Option<&CardSet>;
    fn dummy_side(&self) -> Side{
        self.contract_data().dummy()
    }
    fn hand(&self) -> &CardSet;

}

pub trait RenewableContractInfoSet: ScoringInformationSet<ContractDP>{
    fn renew(&mut self, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>);

}
impl<T: RenewableContractInfoSet> RenewableContractInfoSet for Box<T>{
    fn renew(&mut self, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>) {
        self.as_mut().renew(hand, contract, dummy_hand)
    }
}

pub trait CreatedContractInfoSet: ScoringInformationSet<ContractDP>{
    fn create_new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>, distribution: BiasedHandDistribution) -> Self;
}

impl<T: CreatedContractInfoSet> CreatedContractInfoSet for Box<T>{
    fn create_new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>, distribution: BiasedHandDistribution) -> Self {
        Box::new(T::create_new(side, hand, contract, dummy_hand, distribution))
    }
}

/*
pub trait StandardContractInfoSet: CreatedContractInfoSet + RenewableContractInfoSet{}

impl<T: StandardContractInfoSet> StandardContractInfoSet for Box<T>{}

 */