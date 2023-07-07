use karty::hand::CardSet;
use sztorm::state::agent::InformationSet;
use crate::contract::Contract;
use crate::deal::BiasedHandDistribution;
use crate::player::side::Side;
use crate::sztorm::spec::ContractProtocolSpec;

pub trait RenewableContractInfoSet: InformationSet<ContractProtocolSpec>{
    fn renew(&mut self, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>);

}

pub trait CreatedContractInfoSet: InformationSet<ContractProtocolSpec>{
    fn create_new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>, distribution: BiasedHandDistribution) -> Self;
}