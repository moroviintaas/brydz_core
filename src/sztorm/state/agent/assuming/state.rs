use std::ops::Deref;
use log::debug;
use smallvec::SmallVec;
use karty::cards::Card2SymTrait;
use karty::hand::{CardSet, HandSuitedTrait, HandTrait};
use sztorm::state::agent::{InformationSet, ScoringInformationSet};
use sztorm::state::ConstructedState;
use crate::contract::{Contract, ContractMechanics, ContractParameters};
use crate::deal::{BiasedHandDistribution, DealDistribution, DescriptionDeckDeal};
use crate::error::BridgeCoreError;
use crate::meta::HAND_SIZE;
use crate::player::side::Side;
use crate::sztorm::spec::ContractDP;
use crate::sztorm::state::{ContractAction, ContractInfoSet, ContractStateUpdate, CreatedContractInfoSet, RenewableContractInfoSet, StateWithSide};

#[derive(Debug, Clone)]
pub struct ContractAgentInfoSetAssuming {
    side: Side,
    hand: CardSet,
    dummy_hand: Option<CardSet>,
    contract: Contract,
    card_distribution: BiasedHandDistribution,
}

impl ContractAgentInfoSetAssuming{
    #[allow(dead_code)]
    pub fn new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>, card_distribution: BiasedHandDistribution) -> Self{
        Self{side, hand, dummy_hand, contract, card_distribution}
    }
    #[allow(dead_code)]
    pub fn new_fair(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>) -> Self{
        Self{side, hand, dummy_hand, contract, card_distribution: Default::default()}
    }

    pub fn side(&self) -> &Side{
        &self.side
    }
    pub fn contract(&self) -> &Contract{
        &self.contract
    }
    pub fn hand(&self) -> &CardSet{
        &self.hand
    }
    pub fn dummy_hand(&self) -> Option<&CardSet>{
        self.dummy_hand.as_ref()
    }
    pub fn distribution_assumption(&self) -> &BiasedHandDistribution{
        &self.card_distribution
    }
}



impl InformationSet<ContractDP> for ContractAgentInfoSetAssuming {
    //type ActionType = ContractAction;
    type ActionIteratorType = SmallVec<[ContractAction; HAND_SIZE]>;
    //type Id = Side;
    //type RewardType = u32;

    fn available_actions(&self) -> Self::ActionIteratorType {
        match self.contract.current_side(){
            dec if dec == self.side => {

                match self.contract.current_trick().called_suit(){
                    None => self.hand.into_iter()
                         .map( ContractAction::PlaceCard).collect(),
                    Some(called) => match self.hand.contains_in_suit(&called){
                        true => self.hand.suit_iterator(&called)
                            .map(ContractAction::PlaceCard).collect(),
                        false => self.hand.into_iter()
                            .map(ContractAction::PlaceCard).collect()
                    }
                }
            },
            dummy if dummy == self.side.partner()  && dummy == self.contract.dummy()=> {

                if let Some(dh) = self.dummy_hand{
                    match self.contract.current_trick().called_suit(){
                            None => dh.into_iter()
                                 .map(ContractAction::PlaceCard).collect(),
                            Some(called) => match dh.contains_in_suit(&called){
                                true => dh.suit_iterator(&called)
                                     .map(ContractAction::PlaceCard).collect(),
                                false => dh.into_iter()
                                     .map( ContractAction::PlaceCard).collect()
                            }
                        }
                } else {
                    SmallVec::new()
                }

            },
            _ => SmallVec::new()
        }
    }


    fn is_action_valid(&self, action: &ContractAction) -> bool {
        match action{
            ContractAction::ShowHand(_h) => {
                self.contract.dummy() == self.side
            }
            ContractAction::PlaceCard(c) => match self.hand.contains(c){
                true => match self.contract.current_trick().called_suit(){
                    None => true,
                    Some(s) => {
                        if s == c.suit(){
                            true
                        } else {
                            !self.hand.contains_in_suit(&s)
                        }
                    }
                }
                false => false
            }
        }
    }

    fn update(&mut self, update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
        //debug!("Agent {} received state update: {:?}", self.side, &update);
        let (side, action) = update.into_tuple();
        match action{
            ContractAction::ShowHand(dhand) => match side{
                s if s == self.contract.dummy() => match self.dummy_hand{
                    Some(_) => panic!("Behavior when dummy shows hand second time"),
                    None => {
                        self.dummy_hand = Some(dhand);
                        Ok(())
                    }

                }
                _ => panic!("Non defined behaviour when non dummy shows hand.")

            }
            ContractAction::PlaceCard(card) => {
                let actual_side = match self.contract.dummy() == self.contract.current_side(){
                    false => side,
                    true => match side == self.contract.dummy().partner(){
                        true => self.contract.dummy(),
                        false => side
                    }
                };
                debug!("Agent {:?}: actual_side: {:?}", &self.side, &actual_side);
                self.contract.insert_card(actual_side, card)?;
                if actual_side == self.side{
                    self.hand.remove_card(&card)?
                }
                if actual_side == self.contract.dummy(){
                    if let Some(ref mut dh) = self.dummy_hand{
                        dh.remove_card(&card)?
                    }
                }
                Ok(())

            }
        }
    }


}


impl ScoringInformationSet<ContractDP> for ContractAgentInfoSetAssuming {
    type RewardType = i32;

    fn current_subjective_score(&self) -> Self::RewardType {
        self.contract.total_tricks_taken_axis(self.side.axis()) as i32
    }

    fn penalty_for_illegal() -> Self::RewardType {
        -100
    }
}

impl RenewableContractInfoSet for ContractAgentInfoSetAssuming{
    fn renew(&mut self, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>) {
        self.hand = hand;
        self.contract = contract;
        self.dummy_hand = dummy_hand;
    }
}

impl CreatedContractInfoSet for ContractAgentInfoSetAssuming{
    fn create_new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>, distribution: BiasedHandDistribution) -> Self {
        Self{
            side,
            hand,
            dummy_hand,
            contract,
            card_distribution: distribution
        }
    }
}

impl StateWithSide for ContractAgentInfoSetAssuming{
    fn id(&self) -> Side {
        self.side
    }
}

impl ContractInfoSet for ContractAgentInfoSetAssuming{
    fn side(&self) -> Side {
        self.side
    }

    fn contract_data(&self) -> &Contract {
        &self.contract
    }

    fn dummy_hand(&self) -> Option<&CardSet> {
        self.dummy_hand.as_ref()
    }

    fn hand(&self) -> &CardSet {
        &self.hand
    }
}
impl ConstructedState<ContractDP, (Side,  ContractParameters, DescriptionDeckDeal,)> for ContractAgentInfoSetAssuming{

    fn construct_from(base: (Side, ContractParameters, DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;

         let distr = match descript.probabilities{
            DealDistribution::Fair => Default::default(),
            DealDistribution::Biased(biased) => biased.deref().clone()
        };

        let contract = Contract::new(params);
        Self::new(side, descript.cards[&side] , contract, None, distr)
    }
}
impl ConstructedState<ContractDP, (&Side,  &ContractParameters, &DescriptionDeckDeal,)> for ContractAgentInfoSetAssuming{
    fn construct_from(base: (&Side, &ContractParameters, &DescriptionDeckDeal,)) -> Self {
        let (side, params, descript) = base;

        let distr = match &descript.probabilities{
            DealDistribution::Fair => Default::default(),
            DealDistribution::Biased(biased) => biased.deref().clone()
        };

        let contract = Contract::new(params.clone());
        Self::new(*side, descript.cards[&side], contract, None, distr)
    }
}