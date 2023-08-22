/// ```
/// use brydz_core::bidding::{Bid, Doubling};
/// use brydz_core::cards::trump::TrumpGen;
/// use brydz_core::contract::{Contract, ContractParameters};
/// use brydz_core::contract::SmartTrickSolver::Trump;
/// use brydz_core::player::side::Side::*;
/// use brydz_core::sztorm::state::{ContractAgentInfoSetSimple, ContractStateConverter, ContractStateUpdate};
/// use brydz_core::sztorm::state::ContractAction::PlaceCard;
/// use karty::card_set;
/// use karty::suits::Suit::Diamonds;
/// use karty::cards::*;
/// use sztorm::state::agent::InformationSet;
/// use sztorm_rl::tensor_repr::ConvStateToTensor;
/// let final_bid = Bid::init(TrumpGen::Colored(Diamonds), 3).unwrap();
/// let contract_spec = ContractParameters::new_d(East, final_bid, Doubling::Double);
/// let contract = Contract::new(contract_spec);
/// let whist_hand = card_set!(TWO_CLUBS, SIX_CLUBS, SEVEN_CLUBS, FIVE_DIAMONDS, SEVEN_DIAMONDS,
///     NINE_DIAMONDS, TEN_DIAMONDS, JACK_DIAMONDS, FOUR_HEARTS, SIX_SPADES, SEVEN_SPADES,
///     JACK_SPADES,  QUEEN_SPADES );
/// let dummy_hand = card_set!(EIGHT_CLUBS, THREE_DIAMONDS, EIGHT_DIAMONDS, SEVEN_HEARTS,
///     EIGHT_HEARTS, NINE_HEARTS, QUEEN_HEARTS, ACE_HEARTS, TWO_SPADES, THREE_SPADES, FOUR_SPADES,
///     EIGHT_SPADES, NINE_SPADES);
/// let mut whist_state = ContractAgentInfoSetSimple::new(South, whist_hand, contract, Some(dummy_hand));
/// whist_state.update(ContractStateUpdate::new(South, PlaceCard(JACK_SPADES))).unwrap();
/// whist_state.update(ContractStateUpdate::new(West, PlaceCard(TWO_SPADES))).unwrap();
/// let tensor = ContractStateConverter{}.make_tensor(&whist_state);
/// let v: Vec<f32> = tensor.try_into().unwrap();
/// assert_eq!(v[0], 1.0);
/// assert_eq!(v[1], 1.0);
/// assert_eq!(v[2], 3.0);
/// assert_eq!(v[3], 1.0);
/// assert_eq!(v[4], 0.25);
/// assert_eq!(v[117], 0.25);
/// assert_eq!(v[212], 0.0); //dummy does not have TWO_CLUBS
/// assert_eq!(v[218], 1.0); //dummy has 8 Clubs
/// assert_eq!(v[226], 1.0); //dummy has 3 diamonds
/// assert_eq!(v[264], 1.0); //2 clubs in hand
/// assert_eq!(v[315], 0.0); //A clubs not in hand
/// assert_eq!(v[316], -1.0);
/// assert_eq!(v[317], -1.0);
/// assert_eq!(v[318], 3.0);
/// assert_eq!(v[319], 9.0);
/// assert_eq!(v[320], 3.0);
/// assert_eq!(v[321], 0.0);
/// for i in 322..420{
///     assert_eq!(v[i], -1.0);
/// }
/// ```
pub struct ContractStateConverter{}


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