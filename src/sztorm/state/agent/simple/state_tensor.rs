use std::os::linux::raw::stat;
use tch::Tensor;
use karty::cards::{Card, Card2SymTrait, DECK_SIZE, STANDARD_DECK_CDHS};
use karty::hand::HandTrait;
use karty::symbol::CardSymbol;
use sztorm_rl::tensor_repr::ConvStateToTensor;
use crate::bidding::Doubling;
use crate::contract::ContractMechanics;
use crate::sztorm::state::{ContractAgentInfoSetSimple, ContractStateConverter};

const SPARSE_DECK_SIZE: usize = 52;
const TRICK_REPRESENTATION_SIZE: usize = 2 * 4; //two numbers for suit and figure x 4 5 players
const TRICK_NUMBER: usize = 13;
const CONTRACT_TRUMP_OFFSET: usize = 1;
const CONTRACT_VALUE_OFFSET: usize = CONTRACT_TRUMP_OFFSET + 1;
const DOUBLING_OFFSET: usize = CONTRACT_VALUE_OFFSET + 1;
const DECLARER_DIST_OFFSET: usize = DOUBLING_OFFSET + 1;
const WHIST_DIST_OFFSET: usize = DECLARER_DIST_OFFSET + SPARSE_DECK_SIZE;
const DUMMY_DIST_OFFSET: usize = WHIST_DIST_OFFSET + SPARSE_DECK_SIZE;
const OFFSIDE_DIST_OFFSET: usize = DUMMY_DIST_OFFSET + SPARSE_DECK_SIZE;
const CURRENT_DUMMY_CARDS: usize = OFFSIDE_DIST_OFFSET + SPARSE_DECK_SIZE;
const CURRENT_OWN_CARDS: usize = CURRENT_DUMMY_CARDS + SPARSE_DECK_SIZE;
const TRICKS_OFFSET: usize = CURRENT_OWN_CARDS + SPARSE_DECK_SIZE;
const STATE_REPR_SIZE: usize = TRICKS_OFFSET + 13 * TRICK_REPRESENTATION_SIZE;

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

        let declarer_side = t.contract.declarer();
        let mut state_repr = [0f32; STATE_REPR_SIZE];
        state_repr[0] = (t.side - t.contract.contract_spec().declarer()) as f32;
        state_repr[CONTRACT_TRUMP_OFFSET] = t.contract.contract_spec().bid().trump().into();
        state_repr[CONTRACT_VALUE_OFFSET] = t.contract.contract_spec().bid().number() as f32;
        state_repr[DOUBLING_OFFSET] = match t.contract.contract_spec().doubling(){
            Doubling::None => 0.0,
            Doubling::Double => 1.0,
            Doubling::Redouble => 2.0
        };
        for i in DECLARER_DIST_OFFSET..CURRENT_DUMMY_CARDS{
            state_repr[i] = 0.25;
        }
        if let Some(dhand) = t.dummy_hand{
            for card in STANDARD_DECK_CDHS{
                if dhand.contains(&card){
                    state_repr[CURRENT_DUMMY_CARDS + card.usize_index()] = 1.0;
                }
            }
        } else {
            for i in CURRENT_DUMMY_CARDS..CURRENT_DUMMY_CARDS+DECK_SIZE{
                state_repr[i] = -1.0;
            }
        }
        for card in STANDARD_DECK_CDHS{
            if t.hand.contains(&card){
                state_repr[CURRENT_OWN_CARDS + card.usize_index()] = 1.0;
            }
        }
        let tricks_done = t.contract.completed_tricks().len();
        //setting up completed tricks
        for trick_num in 0..tricks_done{
            let trick = &t.contract.completed_tricks()[trick_num];
            for offset in 0..4{

                state_repr[TRICKS_OFFSET + (trick_num * TRICK_REPRESENTATION_SIZE)  + (offset as usize * 2)]
                    = match trick[declarer_side.next_i(offset)]{
                    None => -1.0,
                    Some(c) => c.suit().usize_index() as f32
                };
                state_repr[TRICKS_OFFSET + (trick_num * TRICK_REPRESENTATION_SIZE)  + (offset as usize * 2) + 1]
                    = match trick[declarer_side.next_i(offset)]{
                    None => -1.0,
                    Some(c) => c.figure().usize_index() as f32
                };
            }

        }
        //setting not completed tricks with -1
        for next_trick_num in tricks_done+1..13{
            for pos in 0..TRICK_REPRESENTATION_SIZE{
                state_repr[TRICKS_OFFSET + (next_trick_num * TRICK_REPRESENTATION_SIZE) + pos] = -1.0;
            }
        }
        //setting current trick
        for offset in 0..4{
            state_repr[TRICKS_OFFSET + (tricks_done * TRICK_REPRESENTATION_SIZE) + (offset as usize * 2)]
                = match t.contract.current_trick()[declarer_side.next_i(offset)]{
                None => -1.0,
                Some(c) => c.suit().usize_index() as f32
            };
            state_repr[TRICKS_OFFSET + (tricks_done * TRICK_REPRESENTATION_SIZE) + (offset as usize * 2) + 1]
                = match t.contract.current_trick()[declarer_side.next_i(offset)]{
                None => -1.0,
                Some(c) => c.figure().usize_index() as f32
            };
        }

        Tensor::from_slice(&state_repr[..])

    }
}