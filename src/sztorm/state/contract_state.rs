use sztorm::state::State;
use crate::player::side::Side;
use crate::sztorm::spec::ContractProtocolSpec;

pub trait ContractState: State<ContractProtocolSpec>{
    fn dummy_side(&self) -> Side;
    fn current_side(&self) -> Side;
    fn is_turn_of_dummy(&self) -> bool{
        self.dummy_side() == self.current_side()
    }
}