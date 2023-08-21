use tch::Tensor;
use sztorm_rl::tensor_repr::ConvStateToTensor;
use crate::sztorm::state::{ContractAgentInfoSetSimple, ContractStateConverter};

const SPARSE_DECK_SIZE: usize = 52;
const TRICK_REPRESENTATION_SIZE: usize = 2 * 4; //two numbers for suit and figure x 4 5 players
const TRICK_NUMBER: usize = 13;

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
impl ConvStateToTensor<ContractAgentInfoSetSimple> for ContractStateConverter{
    fn make_tensor(&self, t: &ContractAgentInfoSetSimple) -> Tensor {
        todo!()
    }
}