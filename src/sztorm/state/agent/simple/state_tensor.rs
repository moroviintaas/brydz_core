use tch::Tensor;
use sztorm_rl::tensor_repr::{ConvertToTensor, ConvStateToTensor};
use crate::sztorm::state::{ContractAgentInfoSetSimple, ContractInfoSetConvert420, ContractInfoSetConvert420Normalised};
use crate::sztorm::state::contract_state_converter_common::*;


//  0000:   ROLE {declarer: 0.0, whist: 1.0, dummy: 2.0, offside: 3.0}
//  0001:   CONTRACT_SUIT {C: 0.0, D: 1.0, H: 2.0, S: 3.0, NT:4.0}
//  0002:   CONTRACT_VALUE: as float (1..=7)
//  0003:   DOUBLING {no: 0.0, double: 1.0, redouble: 2.0}
//  0004:   DECLARER_INIT_DISTRIBUTION [52]
//  0056:   WHIST_INIT_DISTRIBUTION [52]
//  0108:   DUMMY_INIT_DISTRIBUTION [52]
//  0160:   OFFSIDE_INIT_DISTRIBUTION [52]
//  0212:   CURRENT_DUMMY_CARDS [52]
//  0264:   CURRENT_OWN_CARDS [52]
//  0316:   TRICKS [TRICK_NUMBER * TRICK_REPRESENTATION_SIZE]
//              representing trick: [DECLARER[S,F], WHIST[S,F], DUMMY[S,F], OFFSIDE[S,F]] (-1.0, -1.0) for non yet
//  0420:
impl ConvStateToTensor<ContractAgentInfoSetSimple> for ContractInfoSetConvert420 {

    fn make_tensor(&self, t: &ContractAgentInfoSetSimple) -> Tensor {


        let mut state_repr = [0f32; STATE_REPR_SIZE];
        write_contract_params(&mut state_repr, t);
        for i in DECLARER_DIST_OFFSET..CURRENT_DUMMY_CARDS{
            state_repr[i] = 0.25;
        }

        write_current_dummy(&mut state_repr, t);
        write_current_hand(&mut state_repr, t);
        write_tricks(&mut state_repr, t);


        Tensor::from_slice(&state_repr[..])

    }
}

impl ConvStateToTensor<ContractAgentInfoSetSimple> for ContractInfoSetConvert420Normalised{
    fn make_tensor(&self, t: &ContractAgentInfoSetSimple) -> Tensor {


        let mut state_repr = [0f32; STATE_REPR_SIZE];
        write_contract_params_n(&mut state_repr, t);
        for i in DECLARER_DIST_OFFSET..CURRENT_DUMMY_CARDS{
            state_repr[i] = 0.25;
        }

        write_current_dummy(&mut state_repr, t);
        write_current_hand(&mut state_repr, t);
        write_tricks_n(&mut state_repr, t);


        Tensor::from_slice(&state_repr[..])

    }
}

impl ConvertToTensor<ContractInfoSetConvert420> for ContractAgentInfoSetSimple{
    fn to_tensor(&self, way: &ContractInfoSetConvert420) -> Tensor {
        way.make_tensor(self)
    }
}

impl ConvertToTensor<ContractInfoSetConvert420Normalised> for ContractAgentInfoSetSimple{
    fn to_tensor(&self, way: &ContractInfoSetConvert420Normalised) -> Tensor {
        way.make_tensor(self)
    }
}

