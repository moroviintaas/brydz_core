use tch::Tensor;
use karty::cards::Card2SymTrait;
use karty::symbol::CardSymbol;
use sztorm_rl::tensor_repr::{ConvertToTensor, WayToTensor};
use crate::sztorm::state::ContractAction;

#[derive(Default)]
pub struct ContractActionWayToTensor{

}

impl WayToTensor for ContractActionWayToTensor{
    fn desired_shape() -> &'static [i64] {
        &[2]
    }
}

impl ConvertToTensor<ContractActionWayToTensor> for ContractAction{
    fn to_tensor(&self, _way: &ContractActionWayToTensor) -> Tensor {
        match self{
            ContractAction::ShowHand(_) => {panic!("Not prepared to convert show hand to tensor")}
            ContractAction::PlaceCard(c) => {
                let v = vec![c.suit().usize_index() as f32, c.figure().usize_index() as f32];
                Tensor::from_slice(&v[..])
            }
        }
    }
}