use std::fmt::{Debug, Display, Formatter};
use karty::cards::Card;
use karty::hand::CardSet;
use sztorm::{Action};
use sztorm::state::StateUpdate;
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
                ContractAction::ShowHand(h) => write!(f, "Hand|{h:#}|"),
                ContractAction::PlaceCard(c) => write!(f, "{c:#}")
            }
            false => match self {
                ContractAction::ShowHand(h) => write!(f, "Hand|{h:}|"),
                ContractAction::PlaceCard(c) => write!(f, "{c:}")
            }
        }
    }
}

impl Action for ContractAction{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::manual_slice_size_calculation)]
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

#[cfg(feature = "neuro")]
mod tensor{
    use karty::cards::{Card2SymTrait, DECK_SIZE};
    use karty::symbol::CardSymbol;
    use crate::sztorm::state::ContractAction;
    const MIN_ACTION_SIZE:usize = 2;

    impl From<&ContractAction> for [u8;MIN_ACTION_SIZE]{
        fn from(value: &ContractAction) -> Self {
            match value{
                ContractAction::ShowHand(_) => [0,0],
                ContractAction::PlaceCard(c) => [c.suit().position() as u8 +1, c.figure().position() as u8 + 1]
            }
        }
    }
    impl From<&ContractAction> for [f32;MIN_ACTION_SIZE]{
        fn from(value: &ContractAction) -> Self {
            match value{
                ContractAction::ShowHand(_) => [0.0,0.0],
                ContractAction::PlaceCard(c) => [c.suit().position() as f32 +1.0, c.figure().position() as f32 + 1.0]
            }
        }
    }

    impl From<&ContractAction> for tch::Tensor{
        fn from(value: &ContractAction) -> Self {
            tch::Tensor::from_slice(&Into::<[f32;MIN_ACTION_SIZE]>::into(value))
        }
    }

    impl ContractAction{

        pub fn sparse_representation(&self) -> [f32; DECK_SIZE+1]{
            let mut crd = [0.0; DECK_SIZE+1];

            match self{
                ContractAction::ShowHand(h) => {
                    for c in h.into_iter(){
                        crd[c.position()] = 1.0;
                    }
                    crd[DECK_SIZE] = 0.0;
                }
                ContractAction::PlaceCard(c) => {
                    crd[c.position()] = 1.0;
                    crd[DECK_SIZE] = 1.0;
                }
            }

            crd
        }
    }

}