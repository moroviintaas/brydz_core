use tch::Tensor;
use karty::cards::{Card, DECK_SIZE};
use karty::hand::HandTrait;
use karty::symbol::CardSymbol;
use sztorm_rl::tensor_repr::ConvStateToTensor;
use crate::contract::ContractMechanics;
use crate::sztorm::state::{ContractAgentInfoSetAllKnowing, ContractInfoSet, ContractStateConverter};
use crate::sztorm::state::contract_state_converter_common::{DECLARER_DIST_OFFSET, STATE_REPR_SIZE, write_contract_params, write_current_dummy, write_current_hand, write_tricks};

impl ConvStateToTensor<ContractAgentInfoSetAllKnowing> for ContractStateConverter{

    fn make_tensor(&self, t: &ContractAgentInfoSetAllKnowing) -> Tensor {


        let mut state_repr = [0f32; STATE_REPR_SIZE];
        write_contract_params(&mut state_repr, t);
        let declarer_side = t.contract_data().declarer();
        for side_index in 0usize..4{
            for card in Card::iterator(){
                //let proba = t.distribution_assumption()[declarer_side.next_i(side_index as u8)][&card].into();
                if t.initial_deal()[&declarer_side.next_i(side_index as u8)].contains(&card){
                    state_repr[DECLARER_DIST_OFFSET + (DECK_SIZE*side_index) + card.usize_index()] = 1.0;
                }

            }
        }

        write_current_dummy(&mut state_repr, t);
        write_current_hand(&mut state_repr, t);
        write_tricks(&mut state_repr, t);


        Tensor::from_slice(&state_repr[..])

    }
}