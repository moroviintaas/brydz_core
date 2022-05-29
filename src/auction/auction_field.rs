use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use carden::suits::Suit;
use crate::error::{AuctionError, Mismatch};
use crate::error::AuctionError::{BidTooLow, DoubleAfterDouble, DoubleAfterReDouble, DoubleOnSameAxis, DoubleOnVoidCall, ReDoubleAfterReDouble, ReDoubleOnSameAxis, ReDoubleOnVoidCall, ReDoubleWithoutDouble, ViolatedOrder};
use crate::auction::call::{Call, CallEntry, Doubling};
use crate::auction::contract::{Contract};
use crate::auction::bid::{Bid};
use crate::auction::declaration_storage::DeclarationStorage;
use crate::player::side::Side;



#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy ,Clone)]
pub enum AuctionStatus{
    Running(Side),
    Finished,

}


#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct AuctionStack<S: Suit, DS: DeclarationStorage<S>>{
    calls_entries: Vec<CallEntry<S>>,
    current_contract: Option<Contract<S>>,
    declaration_storage: DS,

}

impl<S: Suit, DS: DeclarationStorage<S>> AuctionStack<S, DS>{
    pub fn new() -> Self{
        Self{ calls_entries: Vec::new(), current_contract: None,
            declaration_storage: DS::default()}

    }

    pub fn current_contract(&self) -> Option<&Contract<S>>{
        match &self.current_contract{
            Some(x) => Some(&x),
            None => None
        }

    }

    pub fn last_passes(&self) -> u8{
        let mut counter = 0u8;
        for it in self.calls_entries.iter().rev(){
            match it.call(){
                Call::Pass => {
                    counter +=1;
                },
                _ => break
            }
        }
        counter
    }

    pub fn current_bid(&self) -> Option<&Bid<S>>{
        self.current_contract.as_ref().map(|c| c.bid())
    }

    pub fn add_contract_bid(&mut self, player_side: Side, call: Call<S>) -> Result<AuctionStatus, AuctionError<S>>{
        match self.current_contract{
            None => {
                // First bid, must not be double or redouble
                match call{
                    Call::Pass=> {
                        self.calls_entries.push(CallEntry::new(player_side, call));
                        //self.last_player = Some(player_side);
                        Ok(AuctionStatus::Running(player_side.next()))
                    },
                    Call::Bid(ref bid) => {
                        self.calls_entries.push(CallEntry::new(player_side, call.to_owned()));

                        match self.declaration_storage.get_declarer(player_side.axis(), bid.trump()){
                            None => {
                                self.declaration_storage.set_declarer(player_side, bid.trump().to_owned());
                                self.current_contract = Some(Contract::new(player_side, bid.to_owned() ));
                            },
                            Some(s) => {
                                self.current_contract = Some(Contract::new(s.to_owned(), bid.to_owned() ));
                            }

                        }
                        Ok(AuctionStatus::Running(player_side.next()))
                    }
                    Call::Double => Err(DoubleOnVoidCall),
                    Call::ReDouble => Err(ReDoubleOnVoidCall)


                }
            },
            _ => {
                match player_side{
                    next if next == self.calls_entries.last().unwrap().player_side().next() =>{
                        //good order
                        match call{
                            Call::Pass => match self.last_passes(){
                                0 | 1  => {
                                    self.calls_entries.push(CallEntry::new(player_side, call));
                                    Ok(AuctionStatus::Running(player_side.next()))
                                },
                                _ => {
                                    self.calls_entries.push(CallEntry::new(player_side, call));
                                    Ok(AuctionStatus::Finished)
                                }

                            },
                            Call::Bid(ref bid) => match bid.cmp( self.current_bid().unwrap()){
                                Ordering::Greater => {

                                    match self.declaration_storage.get_declarer(player_side.axis(), bid.trump()){
                                        None => {
                                            self.declaration_storage.set_declarer(player_side, bid.trump().to_owned());
                                            self.current_contract = Some(Contract::new(player_side, bid.to_owned() ));
                                        },
                                        Some(s) => {
                                            self.current_contract = Some(Contract::new(s.to_owned(), bid.to_owned() ));
                                        }

                                    }
                                    self.calls_entries.push(CallEntry::new(player_side, call));

                                    Ok(AuctionStatus::Running(player_side.next()))
                                }
                                _ => Err(BidTooLow(Mismatch{ expected: self.current_bid().unwrap().to_owned(), found:bid.to_owned()}))

                            },
                            Call::Double => match &self.current_contract.as_ref().unwrap().doubling(){
                                Doubling::None => match  self.current_contract.as_ref().unwrap().declarer().axis(){
                                    same if same ==player_side.axis() => Err(DoubleOnSameAxis),
                                    _different => {
                                        //self.current_contract.as_mut().unwrap().doubling() = Doubling::Double;
                                        self.current_contract.as_mut().unwrap().double()?;
                                        self.calls_entries.push(CallEntry::new(player_side, call));

                                        Ok(AuctionStatus::Running(player_side.next()))
                                    }


                                }
                                Doubling::Double => Err(DoubleAfterDouble),
                                Doubling::ReDouble => Err(DoubleAfterReDouble)
                            }
                            Call::ReDouble => match &self.current_contract.as_ref().unwrap().doubling(){
                                Doubling::None => Err(ReDoubleWithoutDouble),
                                Doubling::Double => match self.current_contract.as_ref().unwrap().declarer().axis() {
                                    same if same == player_side.axis() => {
                                        //self.current_contract.as_mut().unwrap().doubling = Doubling::ReDouble;
                                        self.current_contract.as_mut().unwrap().redouble()?;
                                        self.calls_entries.push(CallEntry::new(player_side, call));
                                        Ok(AuctionStatus::Running(player_side.next()))

                                    },
                                    _different => Err(ReDoubleOnSameAxis)
                                },
                                Doubling::ReDouble => Err(ReDoubleAfterReDouble)

                            }
                        }
                    },
                    found => Err(ViolatedOrder(Mismatch{ expected: self.calls_entries.last().unwrap().player_side().next(), found} ))
                }

            }
        }
    }
}
impl<S: Suit, DS: DeclarationStorage<S>> Default for AuctionStack<S, DS> {
     fn default() -> Self {
         Self::new()
     }
}

#[cfg(test)]
mod tests{
    use carden::suits::SuitStd;
    use carden::suits::SuitStd::{Clubs, Diamonds};
    use crate::play::trump::Trump::Colored;
    use crate::error::{AuctionError, Mismatch};
    use crate::error::AuctionError::{BidTooLow, DoubleAfterDouble, DoubleAfterReDouble, ReDoubleAfterReDouble, ReDoubleWithoutDouble};
    use crate::auction::auction_field::{AuctionStack, Contract};
    use crate::player::side::Side::{East, North, South, West};
    use crate::auction::call::{Call, Doubling};
    use crate::auction::bid::{Bid, BID_C1, BID_C2, BID_C3, BID_S2};
    use crate::auction::declaration_storage::GeneralDeclarationStorage;

    #[test]
    fn add_bids_legal(){
        let mut auction_stack = AuctionStack::<SuitStd, GeneralDeclarationStorage<SuitStd>>::new();
        auction_stack.add_contract_bid(East, Call::Pass).unwrap();
        auction_stack.add_contract_bid(South, Call::Pass).unwrap();
        assert_eq!(auction_stack.current_contract, None);
        auction_stack.add_contract_bid(West, Call::Bid(
            Bid::create_bid(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            West,
            Bid::create_bid(Colored(Clubs), 1).unwrap(),
            Doubling::None)
        /*{
            owner: West,
            bid: Bid::create_bid(Colored(Clubs), 1).unwrap(),
            doubling: Doubling::None
        }*/));
        auction_stack.add_contract_bid(North, Call::Bid(
            Bid::create_bid(Colored(Diamonds), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            North,
            Bid::create_bid(Colored(Diamonds), 1).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(East, Call::Pass).unwrap();

        auction_stack.add_contract_bid(South, Call::Bid(
            Bid::create_bid(Colored(Diamonds), 2).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            North,
            Bid::create_bid(Colored(Diamonds), 2).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(West, Call::Double).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            North,
            Bid::create_bid(Colored(Diamonds), 2).unwrap(),
            Doubling::Double)));
        auction_stack.add_contract_bid(North, Call::ReDouble).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            North,
            Bid::create_bid(Colored(Diamonds), 2).unwrap(),
            Doubling::ReDouble)));

    }

    #[test]
    fn violate_auction_order(){
        let mut auction_stack = AuctionStack::<SuitStd, GeneralDeclarationStorage<SuitStd>>::new();
        auction_stack.add_contract_bid(West, Call::Bid(
            Bid::create_bid(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            West,
            Bid::create_bid(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        let r = auction_stack.add_contract_bid(South, Call::Bid(
            Bid::create_bid(Colored(Clubs), 1).unwrap()));
        assert_eq!(r, Err(AuctionError::ViolatedOrder(Mismatch{ expected: North, found: South})));

    }

    #[test]
    fn double_after_double(){
        let mut auction_stack = AuctionStack::<SuitStd, GeneralDeclarationStorage<SuitStd>>::new();
        auction_stack.add_contract_bid(West, Call::Bid(
            Bid::create_bid(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            West,
            Bid::create_bid(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(North, Call::Double).unwrap();
        auction_stack.add_contract_bid(East, Call::Pass).unwrap();
        let r = auction_stack.add_contract_bid(South, Call::Double);
        assert_eq!(r, Err(DoubleAfterDouble));
    }

    #[test]
    fn redouble_after_redouble(){
        let mut auction_stack = AuctionStack::<SuitStd, GeneralDeclarationStorage<SuitStd>>::new();
        auction_stack.add_contract_bid(West, Call::Bid(
            Bid::create_bid(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            West,
            Bid::create_bid(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(North, Call::Double).unwrap();
        auction_stack.add_contract_bid(East, Call::ReDouble).unwrap();
        auction_stack.add_contract_bid(South, Call::Pass).unwrap();
        let r = auction_stack.add_contract_bid(West, Call::ReDouble);
        assert_eq!(r, Err(ReDoubleAfterReDouble));
    }

    #[test]
    fn double_after_redouble(){
        let mut auction_stack = AuctionStack::<SuitStd, GeneralDeclarationStorage<SuitStd>>::new();
        auction_stack.add_contract_bid(West, Call::Bid(
            Bid::create_bid(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            West,
            Bid::create_bid(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(North, Call::Double).unwrap();
        auction_stack.add_contract_bid(East, Call::ReDouble).unwrap();
        let r = auction_stack.add_contract_bid(South, Call::Double);
        assert_eq!(r, Err(DoubleAfterReDouble));
    }

    #[test]
    fn redouble_without_double(){
        let mut auction_stack = AuctionStack::<SuitStd, GeneralDeclarationStorage<SuitStd>>::new();
        auction_stack.add_contract_bid(West, Call::Bid(
            Bid::create_bid(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(Contract::new_d(
            West,
            Bid::create_bid(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        let r = auction_stack.add_contract_bid(North, Call::ReDouble);
        assert_eq!(r, Err(ReDoubleWithoutDouble));
    }

    #[test]
    fn bid_too_low(){
        let mut auction_stack = AuctionStack::<SuitStd, GeneralDeclarationStorage<SuitStd>>::new();
        auction_stack.add_contract_bid(West, Call::Bid(
            Bid::create_bid(Colored(Clubs), 2).unwrap())).unwrap();

        let r = auction_stack.add_contract_bid(North, Call::Bid(
            Bid::create_bid(Colored(Diamonds), 1).unwrap()));
        assert_eq!(r, Err(BidTooLow(Mismatch{
            expected: Bid::create_bid(Colored(Clubs), 2).unwrap(),
            found: Bid::create_bid(Colored(Diamonds),1).unwrap() })));
    }

    #[test]
    fn declarer_simple(){
        let mut auction_stack = AuctionStack::<SuitStd, GeneralDeclarationStorage<SuitStd>>::new();
        auction_stack.add_contract_bid(West, Call::Bid(BID_C1)).unwrap();
        auction_stack.add_contract_bid(North, Call::Bid(BID_C2)).unwrap();
        auction_stack.add_contract_bid(East, Call::Bid(BID_S2)).unwrap();

        assert_eq!(auction_stack.current_contract().unwrap().declarer(), East);

    }

    #[test]
    fn declarer_partner(){
        let mut auction_stack = AuctionStack::<SuitStd, GeneralDeclarationStorage<SuitStd>>::new();
        auction_stack.add_contract_bid(West, Call::Bid(BID_C1)).unwrap();
        auction_stack.add_contract_bid(North, Call::Bid(BID_C2)).unwrap();
        auction_stack.add_contract_bid(East, Call::Bid(BID_C3)).unwrap();

        assert_eq!(auction_stack.current_contract().unwrap().declarer(), West);

    }






}