use std::thread;
use karty::hand::CardSet;
use karty::suits::Suit::Spades;
use sztorm::RandomPolicy;
use sztorm::automatons::rr::{AgentRR, EnvironmentRR};
use crate::bidding::Bid;
use crate::cards::trump::TrumpGen;
use crate::contract::{Contract, ContractParametersGen};
use crate::deal::fair_bridge_deal;
use crate::player::side::{Side, SideMap};
use crate::player::side::Side::*;
use crate::sztorm::agent::ContractAgent;
use crate::sztorm::comm::ContractEnvSyncComm;
use crate::sztorm::env::ContractEnv;
use crate::sztorm::state::{ContractAgentStateMin, ContractDummyState, ContractEnvStateMin};

mod env_agent;

#[test]
fn random_agents_sync_comm(){
    let contract = ContractParametersGen::new(Side::East, Bid::init(TrumpGen::Colored(Spades), 2).unwrap());
    let (comm_env_north, comm_north) = ContractEnvSyncComm::new_pair();
    let (comm_env_east, comm_east) = ContractEnvSyncComm::new_pair();
    let (comm_env_west, comm_west) = ContractEnvSyncComm::new_pair();
    let (comm_env_south, comm_south) = ContractEnvSyncComm::new_pair();

    let comm_assotiation = SideMap::new(comm_env_north, comm_env_east, comm_env_south, comm_env_west);
    let initial_contract = Contract::new(contract);

    let env_initial_state = ContractEnvStateMin::new(initial_contract.clone(), None);
    let mut simple_env = ContractEnv::new(env_initial_state, comm_assotiation);

    let card_deal = fair_bridge_deal::<CardSet>();
    let (hand_north, hand_east, hand_south, hand_west) = card_deal.destruct();

    let initial_state_east = ContractAgentStateMin::new(East, hand_east, initial_contract.clone(), None);
    let initial_state_south = ContractAgentStateMin::new(South, hand_south, initial_contract.clone(), None);
    let initial_state_west = ContractDummyState::new(West, hand_west, initial_contract.clone());
    let initial_state_north = ContractAgentStateMin::new(North, hand_north, initial_contract.clone(), None);


    let random_policy = RandomPolicy::<ContractAgentStateMin>::new();
    let policy_dummy = RandomPolicy::<ContractDummyState>::new();

    let mut agent_east = ContractAgent::new(initial_state_east, comm_east, random_policy.clone() );
    let mut agent_south = ContractAgent::new(initial_state_south, comm_south, random_policy.clone() );
    let mut agent_west = ContractAgent::new(initial_state_west, comm_west, policy_dummy);
    let mut agent_north = ContractAgent::new(initial_state_north, comm_north, random_policy );

    thread::scope(|s|{
        s.spawn(||{
            simple_env.env_run_rr().unwrap();
        });
        s.spawn(||{
            agent_east.run_rr().unwrap();
        });

        s.spawn(||{
            agent_south.run_rr().unwrap();
        });

        s.spawn(||{
            agent_west.run_rr().unwrap();
        });

        s.spawn(||{
            agent_north.run_rr().unwrap();
        });
    })

}