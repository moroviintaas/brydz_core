use std::fmt::{Debug, Display, Formatter};
use karty::cards::Card;
use karty::hand::CardSet;
use tur::action::Action;
use tur::state::StateUpdate;
use crate::player::side::Side;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum ContractAction{
    ShowHand(CardSet),
    PlaceCard(Card)
}

impl Display for ContractAction{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.alternate(){
            true => match self{
                ContractAction::ShowHand(h) => write!(f, "Hand|{:#}|", h),
                ContractAction::PlaceCard(c) => write!(f, "{:#}", c)
            }
            false => match self {
                ContractAction::ShowHand(h) => write!(f, "Hand|{:}|", h),
                ContractAction::PlaceCard(c) => write!(f, "{:}", c)
            }
        }
    }
}

impl Action for ContractAction{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub struct ContractStateUpdate {
    agent: Side,
    action: ContractAction


}

impl ContractStateUpdate{
    pub fn new(side: Side, action: ContractAction) -> Self{
        Self{agent:side, action}
    }

    pub fn side(&self) -> &Side{
        &self.agent
    }
    pub fn action(&self) -> &ContractAction{
        &self.action
    }
    pub fn into_tuple(self) -> (Side, ContractAction){
        (self.agent, self.action)
    }
}


impl Display for ContractStateUpdate{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.alternate(){
            true => write!(f, "agent {:#}; action: {:#}", &self.agent, &self.action),
            false => write!(f, "agent {}; action: {}", &self.agent, &self.action)
        }
    }
}



impl StateUpdate for ContractStateUpdate{

}