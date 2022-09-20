use log::{debug, error, info};
use crate::deal::DealMaintainer;
use crate::error::{BridgeErrorStd, DealError, FlowError};
use crate::player::situation::Situation;
use crate::protocol::{ClientDealMessage, DealAction, DealNotify, ServerDealMessage};
use crate::protocol::ClientControlMessage::{ClientBridgeError, IamReady, Quit};
use crate::world::agent::{AutomaticAgent, AwareAgent, CommunicatingAgent};

impl<Player: CommunicatingAgent<ServerDealMessage, ClientDealMessage, DealAction, BridgeErrorStd> + AwareAgent<Situation>>
AutomaticAgent<BridgeErrorStd> for Player{


    fn run(&mut self) -> Result<(), BridgeErrorStd> {
        self.send(IamReady.into())?;
        loop{
            match self.recv(){
                Ok(message) => match message {
                    ServerDealMessage::Notify(notify) => match notify{
                        DealNotify::CardPlayed(side, card) => {
                            debug!("{:?} received info that player {:?}, played {:#}.", self.env().side(), side, card);
                            if let Err(e) = self.env_mut().mark_card_used(side, card){
                                error!("{:?} encountered error: {:?}", self.env().side(), e.clone());
                                self.send(ClientBridgeError(e.into()).into())?;

                            }

                        }
                        DealNotify::TrickClosed(_) => {}
                        DealNotify::CardAccepted(card) => {
                            debug!("Player {:?} received info that card {:?}, was accepted.", self.env().side(), card);
                        }
                        DealNotify::CardDeclined(card) => {
                            self.send(Quit.into())?;
                            return Err(DealError::DuplicateCard(card).into());
                        }
                        DealNotify::DummyPlacedHand(hand) => {
                            debug!("Declarer {:?} received message with dummy's hand: {:?}", &self.env().side(), &hand);
                            if self.env().dummy_hand().cards().is_empty(){
                                self.set_dummy_hand(hand);//env_mut().set_dummy(hand);
                            }
                            else{
                                self.send(ClientBridgeError(FlowError::ConfusingMessage.into()).into())?;
                                self.send(Quit.into())?
                            }
                        }
                        DealNotify::YourMove => {
                            debug!("Player {:?} received signal to move.", self.env().side());
                            debug!("Player {:?}'s hand: {:#}", self.env().side(), self.env().hand());
                            if self.env().side() == self.env().deal().declarer() || self.env().side() == self.env().deal().declarer().partner(){
                                debug!("Player ({:?}) dummy_hand: {:#}", self.env().side(), self.env().dummy_hand());
                            }

                            //self.make_move(&mut rng)?;
                            match self.select_action(){
                                Err(e) => {
                                    error!("Player {:?} couldn't determine action. Player's hand: {:#}. ", self.env().side(), self.env().hand());
                                    if self.env().side() == self.env().deal().declarer() || self.env().side() == self.env().deal().declarer().partner(){
                                        error!("Player {:?} couldn't determine action. Dummy's hand: {:#}. ", self.env().side(), self.env().dummy_hand());
                                    }
                                    self.send(ClientBridgeError(e).into()).unwrap_or(());
                                }
                                Ok(action) => match action{
                                    DealAction::PlayCard(card) => {
                                        debug!("Player {:?} selected {:#} card to play.", self.env().side(), card);
                                        self.send(action.into()).unwrap_or(());
                                    }
                                }
                            }
                        }
                        DealNotify::DealClosed => {
                            self.send(Quit.into()).unwrap_or(());
                            return Ok(());
                        }
                    }
                    ServerDealMessage::Info(_) => {}
                    ServerDealMessage::Control(_) => {}
                }
                Err(e) => {
                    info!("Received communication error: {:?}", e);
                    return Err(e)

                }
            }
        }

    }
}