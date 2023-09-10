use std::fmt::{Display, Formatter};
use karty::cards::Card;
use karty::hand::CardSet;

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


#[cfg(feature = "neuro")]
mod neuro_impls{
    use std::fmt::{Display, Formatter};
    use tch::Tensor;
    use karty::cards::{Card, Card2SGen};
    use karty::error::CardError;
    use karty::figures::Figure;
    use karty::suits::Suit;
    use karty::symbol::CardSymbol;
    use sztorm::Action;
    use sztorm::error::ConvertError;
    use sztorm_rl::tensor_repr::ActionTensor;
    use crate::sztorm::state::ContractAction;




    impl ActionTensor for ContractAction{
        fn to_tensor(&self) -> Tensor {
            match self{
                ContractAction::ShowHand(h) => panic!("Show hand is not expected to be converted to tensor - this is exclusive move of dummy"),
                ContractAction::PlaceCard(c) => Tensor::from_slice(&[c.usize_index() as f32;1])
            }
        }

        fn try_from_tensor(t: &Tensor) -> Result<Self, ConvertError> {
            let v: Vec<i64> = match Vec::try_from(t){
                Ok(v) => v,
                Err(e) => {
                    return Err(ConvertError::ActionDeserialize(format!("{}", t)))
                }
            };
            let action_index = v[0];
            Card::from_usize_index(action_index as usize)
                .map_err(|e| ConvertError::ActionDeserialize(format!("Bad index of card: {e:}")))
                .map(|ok| Self::PlaceCard(ok))

        }
    }
}

#[cfg(feature = "neuro")]
pub use neuro_impls::*;
use sztorm::Action;