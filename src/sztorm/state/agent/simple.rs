use smallvec::SmallVec;
use karty::hand::{HandSuitedTrait, HandTrait, CardSet};
use crate::contract::{Contract, ContractMechanics};
use crate::error::BridgeCoreError;
use crate::meta::HAND_SIZE;
use crate::player::side::Side;
use crate::sztorm::state::{ContractAction, ContractStateUpdate};
use log::debug;
use karty::cards::Card2SymTrait;
use crate::sztorm::spec::ContractProtocolSpec;

#[derive(Debug, Clone)]
pub struct ContractAgentInfoSetSimple {
    side: Side,
    hand: CardSet,
    dummy_hand: Option<CardSet>,
    contract: Contract
}

impl ContractAgentInfoSetSimple {
    pub fn new(side: Side, hand: CardSet, contract: Contract, dummy_hand: Option<CardSet>) -> Self{
        Self{side, hand, dummy_hand, contract}
    }
}


impl sztorm::State<ContractProtocolSpec> for ContractAgentInfoSetSimple {
    //type UpdateType = ContractStateUpdate;
    //type Error = BridgeCoreError;

    fn update(&mut self, update: ContractStateUpdate) -> Result<(), BridgeCoreError> {
        //debug!("Agent {} received state update: {:?}", self.side, &update);
        let (side, action) = update.into_tuple();
        match action{
            ContractAction::ShowHand(dhand) => match side{
                s if s == self.contract.dummy() => match self.dummy_hand{
                    Some(_) => panic!("Behavior when dummy shows hand second time"),
                    None => {
                        self.dummy_hand = Some(dhand);
                        Ok(())
                    }

                }
                _ => panic!("Non defined behaviour when non dummy shows hand.")

            }
            ContractAction::PlaceCard(card) => {
                let actual_side = match self.contract.dummy() == self.contract.current_side(){
                    false => side,
                    true => match side == self.contract.dummy().partner(){
                        true => self.contract.dummy(),
                        false => side
                    }
                };
                debug!("Agent {:?}: actual_side: {:?}", &self.side, &actual_side);
                self.contract.insert_card(actual_side, card)?;
                if actual_side == self.side{
                    self.hand.remove_card(&card)?
                }
                if actual_side == self.contract.dummy(){
                    if let Some(ref mut dh) = self.dummy_hand{
                        dh.remove_card(&card)?
                    }
                }
                Ok(())

            }
        }
    }

    fn is_finished(&self) -> bool {
        self.contract.is_completed()
    }
}

impl sztorm::InformationSet<ContractProtocolSpec> for ContractAgentInfoSetSimple {
    //type ActionType = ContractAction;
    type ActionIteratorType = SmallVec<[ContractAction; HAND_SIZE]>;
    //type Id = Side;
    type RewardType = u32;

    fn available_actions(&self) -> Self::ActionIteratorType {
        match self.contract.current_side(){
            dec if dec == self.side => {

                match self.contract.current_trick().called_suit(){
                    None => self.hand.into_iter()
                         .map( ContractAction::PlaceCard).collect(),
                    Some(called) => match self.hand.contains_in_suit(&called){
                        true => self.hand.suit_iterator(&called)
                            .map(ContractAction::PlaceCard).collect(),
                        false => self.hand.into_iter()
                            .map(ContractAction::PlaceCard).collect()
                    }
                }
            },
            dummy if dummy == self.side.partner()  && dummy == self.contract.dummy()=> {

                if let Some(dh) = self.dummy_hand{
                    match self.contract.current_trick().called_suit(){
                            None => dh.into_iter()
                                 .map(ContractAction::PlaceCard).collect(),
                            Some(called) => match dh.contains_in_suit(&called){
                                true => dh.suit_iterator(&called)
                                     .map(ContractAction::PlaceCard).collect(),
                                false => dh.into_iter()
                                     .map( ContractAction::PlaceCard).collect()
                            }
                        }
                } else {
                    SmallVec::new()
                }

            },
            _ => SmallVec::new()
        }
    }

    fn id(&self) -> &Side {
        &self.side
    }

    fn is_action_valid(&self, action: &ContractAction) -> bool {
        match action{
            ContractAction::ShowHand(_h) => {
                self.contract.dummy() == self.side
            }
            ContractAction::PlaceCard(c) => match self.hand.contains(c){
                true => match self.contract.current_trick().called_suit(){
                    None => true,
                    Some(s) => {
                        if s == c.suit(){
                            true
                        } else {
                            !self.hand.contains_in_suit(&s)
                        }
                    }
                }
                false => false
            }
        }
    }

    fn current_score(&self) -> Self::RewardType {
        self.contract.total_tricks_taken_axis(self.side.axis())
    }
}


#[cfg(feature = "dl")]
mod tensor{
    //use tensorflow::{QUInt8, Tensor};
    use crate::sztorm::state::ContractAgentInfoSetSimple;
    use karty::cards::{Card2SymTrait, DECK_SIZE, STANDARD_DECK_CDHS};
    use karty::hand::{ HandTrait};
    use karty::register::Register;
    use karty::symbol::CardSymbol;
    use crate::bidding::Doubling;
    use crate::cards::trump::TrumpGen;
    use crate::contract::ContractMechanics;
    //use crate::meta::DECK_SIZE;



    const SURE: u8 = 120;
    const ONE_IN_TWO: u8 = SURE/2;
    const ONE_IN_THREE: u8 = SURE/3;

    const TRICK_STARTER: usize = DECK_SIZE * 4;
    const TRICK_COMPLETION: usize = TRICK_STARTER + 1;
    const OWN_CARD: usize = TRICK_COMPLETION + 1;
    const LEFT_CARD: usize = OWN_CARD + 2;

    const PARTNER_CARD: usize = LEFT_CARD + 2;
    const RIGHT_CARD: usize = PARTNER_CARD + 2;
    const BID_OFFSET: usize = RIGHT_CARD + 2;
    const PLAY_AS_DUMMY: usize = BID_OFFSET + 3;



    const SIMPLE_INFO_SET_LENGTH:usize = PLAY_AS_DUMMY + 1;



    impl From<&ContractAgentInfoSetSimple> for [u8;SIMPLE_INFO_SET_LENGTH] {
        fn from(state: &ContractAgentInfoSetSimple) -> Self {
            let dummy_offset = (state.contract.dummy() - state.side) as usize;

            let unknown_side_1 = state.side.first_unknown_side(state.contract.declarer());
            let unknown_side_2 = state.side.second_unknown_side(state.contract.declarer());
            let unknown_offset_1 = (unknown_side_1 - state.side) as usize;
            let unknown_offset_2 = (unknown_side_2 - state.side) as usize;

            //let mut array:[QUInt8;SIMPLE_INFO_SET_LENGTH] = [QUInt8::zero();SIMPLE_INFO_SET_LENGTH];
            let mut array = [0; SIMPLE_INFO_SET_LENGTH];
            array[TRICK_STARTER] = state.contract.declarer() - state.side;
            array[TRICK_COMPLETION] = state.contract.current_trick().count_cards();
            (array[LEFT_CARD], array[LEFT_CARD+1]) = match state.contract.current_trick()[state.side]{
                None => (0, 0),
                Some(c) => (c.suit().position() as u8 + 1, c.figure().position() as u8 + 1)
            };
            (array[LEFT_CARD], array[LEFT_CARD+1]) = match state.contract.current_trick()[state.side.next()]{
                None => (0, 0),
                Some(c) => (c.suit().position() as u8 + 1, c.figure().position() as u8 + 1)
            };
            (array[PARTNER_CARD], array[PARTNER_CARD+1]) = match state.contract.current_trick()[state.side.partner()]{
                None => (0, 0),
                Some(c) => (c.suit().position() as u8 + 1, c.figure().position() as u8 + 1)
            };
            (array[RIGHT_CARD], array[RIGHT_CARD+1]) = match state.contract.current_trick()[state.side.prev()]{
                None => (0, 0),
                Some(c) => (c.suit().position() as u8 + 1, c.figure().position() as u8 + 1)
            };

            array[BID_OFFSET] = match state.contract.contract_spec().bid().trump(){
                    TrumpGen::Colored(s) => s.position() as u8 + 1,
                    TrumpGen::NoTrump => 0
            };
            array[BID_OFFSET+1] = state.contract.contract_spec().bid().number();
            array[BID_OFFSET+2] = match state.contract.contract_spec().doubling(){
                Doubling::None => 0,
                Doubling::Double => 1,
                Doubling::Redouble => 2
            };
            array[PLAY_AS_DUMMY] = match state.contract.current_side() == state.contract.dummy(){
                true => 1,
                false => 0
            };
            for card in STANDARD_DECK_CDHS{
                if state.hand.contains(&card){
                   array[card.position()] = SURE; //sure
                   /*not needed
                   for i in 1..=3{
                       array[(i*DECK_SIZE)+card.position()] = QUInt8::from(0);
                   }*/
                } else if !state.contract.used_cards().is_registered(&card){
                    match state.dummy_hand{
                        None => {
                            //dummy's hand not shown yet
                            for i in 1..=3{
                                array[(i*DECK_SIZE) + card.position()] = ONE_IN_THREE;
                            }
                        }
                        Some(dhand) => {
                            if dhand.contains(&card){
                                array[(DECK_SIZE*dummy_offset) + card.position()] = SURE;
                            } else {
                                //this is tricky
                                if state.contract.suits_exhausted().is_registered(&(unknown_side_1, card.suit())){
                                    array[(DECK_SIZE*unknown_offset_2) + card.position()] = SURE;
                                }
                                else if state.contract.suits_exhausted().is_registered(&(unknown_side_2, card.suit())){
                                    array[(DECK_SIZE*unknown_offset_1) + card.position()] = SURE;
                                }
                                else{
                                    array[(DECK_SIZE*unknown_offset_1) + card.position()] = ONE_IN_TWO;
                                    array[(DECK_SIZE*unknown_offset_2) + card.position()] = ONE_IN_TWO;
                                }

                            }
                        }
                    }
                    //card was not yet played
                } else {
                    //card was played before
                }
            }
            array


        }
    }
    impl From<&ContractAgentInfoSetSimple> for [f32;SIMPLE_INFO_SET_LENGTH] {
        fn from(state: &ContractAgentInfoSetSimple) -> Self {
            let dummy_offset = (state.contract.dummy() - state.side) as usize;

            let unknown_side_1 = state.side.first_unknown_side(state.contract.declarer());
            let unknown_side_2 = state.side.second_unknown_side(state.contract.declarer());
            let unknown_offset_1 = (unknown_side_1 - state.side) as usize;
            let unknown_offset_2 = (unknown_side_2 - state.side) as usize;

            //let mut array:[QUInt8;SIMPLE_INFO_SET_LENGTH] = [QUInt8::zero();SIMPLE_INFO_SET_LENGTH];
            let mut array = [0.0; SIMPLE_INFO_SET_LENGTH];
            array[TRICK_STARTER] = (state.contract.declarer() - state.side) as f32;
            array[TRICK_COMPLETION] = (state.contract.current_trick().count_cards()) as f32;
            (array[LEFT_CARD], array[LEFT_CARD+1]) = match state.contract.current_trick()[state.side]{
                None => (0.0, 0.0),
                Some(c) => (c.suit().position() as f32 + 1.0, c.figure().position() as f32 + 1.0)
            };
            (array[LEFT_CARD], array[LEFT_CARD+1]) = match state.contract.current_trick()[state.side.next()]{
                None => (0.0, 0.0),
                Some(c) => (c.suit().position() as f32 + 1.0, c.figure().position() as f32 + 1.0)
            };
            (array[PARTNER_CARD], array[PARTNER_CARD+1]) = match state.contract.current_trick()[state.side.partner()]{
                None => (0.0, 0.0),
                Some(c) => (c.suit().position() as f32 + 1.0, c.figure().position() as f32 + 1.0)
            };
            (array[RIGHT_CARD], array[RIGHT_CARD+1]) = match state.contract.current_trick()[state.side.prev()]{
                None => (0.0, 0.0),
                Some(c) => (c.suit().position() as f32 + 1.0, c.figure().position() as f32 + 1.0)
            };

            array[BID_OFFSET] = match state.contract.contract_spec().bid().trump(){
                    TrumpGen::Colored(s) => s.position() as f32 + 1.0,
                    TrumpGen::NoTrump => 0.0
            };
            array[BID_OFFSET+1] = state.contract.contract_spec().bid().number() as f32;
            array[BID_OFFSET+2] = match state.contract.contract_spec().doubling(){
                Doubling::None => 0.0,
                Doubling::Double => 1.0,
                Doubling::Redouble => 2.0
            };
            array[PLAY_AS_DUMMY] = match state.contract.current_side() == state.contract.dummy(){
                true => 1.0,
                false => 0.0
            };
            for card in STANDARD_DECK_CDHS{
                if state.hand.contains(&card){
                   array[card.position()] = 1.0; //sure
                   /*not needed
                   for i in 1..=3{
                       array[(i*DECK_SIZE)+card.position()] = QUInt8::from(0);
                   }*/
                } else if !state.contract.used_cards().is_registered(&card){
                    match state.dummy_hand{
                        None => {
                            //dummy's hand not shown yet
                            for i in 1..=3{
                                array[(i*DECK_SIZE) + card.position()] = 1.0/3.0;
                            }
                        }
                        Some(dhand) => {
                            if dhand.contains(&card){
                                array[(DECK_SIZE*dummy_offset) + card.position()] = 1.0;
                            } else {
                                //this is tricky
                                if state.contract.suits_exhausted().is_registered(&(unknown_side_1, card.suit())){
                                    array[(DECK_SIZE*unknown_offset_2) + card.position()] = 1.0;
                                }
                                else if state.contract.suits_exhausted().is_registered(&(unknown_side_2, card.suit())){
                                    array[(DECK_SIZE*unknown_offset_1) + card.position()] = 1.0;
                                }
                                else{
                                    array[(DECK_SIZE*unknown_offset_1) + card.position()] = 0.5;
                                    array[(DECK_SIZE*unknown_offset_2) + card.position()] = 0.5;
                                }

                            }
                        }
                    }
                    //card was not yet played
                } else {
                    //card was played before
                }
            }
            array


        }
    }


    impl From<&ContractAgentInfoSetSimple> for tch::Tensor{
        fn from(value: &ContractAgentInfoSetSimple) -> Self {
            tch::Tensor::of_slice(&Into::<[f32;SIMPLE_INFO_SET_LENGTH]>::into(value))
        }
    }



    /*
    impl From<&ContractAgentInfoSetSimple> for Tensor<QUInt8>{
        fn from(value: &ContractAgentInfoSetSimple) -> Self {
            let array:[u8;SIMPLE_INFO_SET_LENGTH] = value.into();
            Tensor::from(array.map(|b| QUInt8::from(b)))
        }
    }

     */
}


#[cfg(test)]
mod tests{
    use std::str::FromStr;
    use karty::card_set;
    use karty::cards::{ACE_DIAMONDS, EIGHT_DIAMONDS, FIVE_DIAMONDS, KING_SPADES};
    use karty::hand::CardSet;
    use karty::suits::Suit::Hearts;
    use sztorm::State;
    use crate::bidding::Bid;
    use crate::cards::trump::TrumpGen;
    use crate::contract::{Contract, ContractParametersGen};
    use crate::player::side::Side::{*};
    use crate::sztorm::state::{ContractAgentInfoSetSimple, ContractStateUpdate};
    use crate::sztorm::state::ContractAction::{PlaceCard, ShowHand};

    #[test]
    fn convert_simple_info_set_to_bytes(){
        let contract = Contract::new(
            ContractParametersGen::new(
                East,
                Bid::init(TrumpGen::Colored(Hearts), 2).unwrap() ));
        let mut info_set = ContractAgentInfoSetSimple::new(North,
                                                       CardSet::from_str("AT86.KJT93.4T.2A").unwrap(),
                                                       contract, None);

        let state_as_vec:[u8; 222] = (&info_set).into();
        assert_eq!(Vec::from(state_as_vec),
                   vec![120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120,//north
                        0, 0, 120, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0,
                        0, 120, 0, 0, 0, 0, 0, 120, 120, 120, 0, 120, 0,
                        0, 0, 0, 0, 120, 0, 120, 0, 120, 0, 0, 0, 120,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//east (declarer)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 40,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//south (partner)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 40,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//west (dummy)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 40,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 0

                   ]
        );

        info_set.update(ContractStateUpdate::new(South, PlaceCard(ACE_DIAMONDS))).unwrap();
        let state_as_vec:[u8;222] = (&info_set).into();
        assert_eq!(Vec::from(state_as_vec),
                   vec![120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120,//north
                        0, 0, 120, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0,
                        0, 120, 0, 0, 0, 0, 0, 120, 120, 120, 0, 120, 0,
                        0, 0, 0, 0, 120, 0, 120, 0, 120, 0, 0, 0, 120,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//east (declarer)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 0,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//south (partner)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 0,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        0, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 0,//west (dummy)
                        40, 40, 0, 40, 40, 40, 40, 40, 0, 40, 40, 40, 0,
                        40, 0, 40, 40, 40, 40, 40, 0, 0, 0, 40, 0, 40,
                        40, 40, 40, 40, 0, 40, 0, 40, 0, 40, 40, 40, 0,
                        1, 1, 0, 0, 0, 0, 2, 13, 0, 0 , 3, 2, 0, 1

                   ]
        );
        //AT86.KJT93.4T.2A
        info_set.update(ContractStateUpdate::new(West,
                                                 ShowHand(CardSet::from_str("QJ3.8764.A95.T96").unwrap()))).unwrap();
        info_set.update(ContractStateUpdate::new(West,
                                                 PlaceCard(FIVE_DIAMONDS))).unwrap();

        let state_as_vec:[u8;222] = (&info_set).into();
        assert_eq!(Vec::from(state_as_vec),
                   vec![120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120,//north
                        0, 0, 120, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0,
                        0, 120, 0, 0, 0, 0, 0, 120, 120, 120, 0, 120, 0,
                        0, 0, 0, 0, 120, 0, 120, 0, 120, 0, 0, 0, 120,
                        0, 60, 60, 60, 0, 60, 60, 0, 0, 60, 60, 60, 0,//east (declarer)
                        60, 60, 0, 0, 60, 60, 60, 0, 0, 60, 60, 60, 0,
                        60, 0, 0, 60, 0, 0, 0, 0, 0, 0, 60, 0, 60,
                        60, 0, 60, 60, 0, 60, 0, 60, 0, 0, 0, 60, 0,
                        0, 60, 60, 60, 0, 60, 60, 0, 0, 60, 60, 60, 0,//south (partner)
                        60, 60, 0, 0, 60, 60, 60, 0, 0, 60, 60, 60, 0,
                        60, 0, 0, 60, 0, 0, 0, 0, 0, 0, 60, 0, 60,
                        60, 0, 60, 60, 0, 60, 0, 60, 0, 0, 0, 60, 0,
                        0, 0, 0, 0, 120, 0, 0, 120, 120, 0, 0, 0, 0,//west (dummy)
                        0, 0, 0, 0, 0, 0, 0, 120, 0, 0, 0, 0, 0,
                        0, 0, 120, 0, 120, 120, 120, 0, 0, 0, 0, 0, 0,
                        0, 120, 0, 0, 0, 0, 0, 0, 0, 120, 120, 0, 0,
                        1, 2, 0, 0, 0, 0, 2, 13, 2, 4 , 3, 2, 0, 0

                   ]
        );




    }
}