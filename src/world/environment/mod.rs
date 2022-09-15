
use std::error::Error;
use log::{error, info, warn};

mod channel;
mod rr_env;
pub use rr_env::*;

pub use channel::*;


use karty::cards::CardStd;
use crate::deal::{DealMaintainer, RegDealStd};
use crate::distribution::hand::BridgeHand;
use crate::error::{BridgeErrorStd, CommError, FlowError};
use crate::player::side::{Side, SIDES};
use crate::protocol::{ClientControlMessage, ClientDealInformation, ClientDealMessage, DealAction, ServerDealMessage};
use crate::protocol::DealNotify::{CardAccepted, CardPlayed, DealClosed, DummyPlacedHand, TrickClosed, YourMove};
use crate::protocol::ServerControlMessage::{GameOver, PlayerLeft, ServerBridgeError, ServerStopping};

pub trait OrderGuard{
    fn current_side(&self) -> Option<Side>;
    fn is_dummy_placed(&self) -> bool;
}

pub trait CardCheck<E: Error>{
    fn check(&self, card: &CardStd) -> Result<(), E>;

}

pub trait Environment{
    fn deal(&self) -> &RegDealStd;
    fn deal_mut(&mut self) -> &mut RegDealStd;
    fn card_check(&self, card: &CardStd) -> Result<(), BridgeErrorStd>;
    fn dummy_hand(&self) -> Option<&BridgeHand>;
    fn set_dummy_hand(&mut self, hand: BridgeHand);
    fn next_player(&self) -> Option<Side>;
}

pub trait WaitReady{
    fn are_players_ready(&self) -> bool;
    fn set_ready(&mut self, side: &Side);
}

pub trait AutomaticEnvironment<E: Error>{
    fn run(&mut self) -> Result<(), E>;
}
pub trait CommunicatingEnvironment<Sm, Cm, E:Error>{
    fn send(&self, side: &Side, message: Sm) -> Result<(), E>;
    fn send_to_all(&self, message: Sm) -> Result<(), E>;
    fn recv(&mut self, side: &Side) -> Result<Cm, E>;
    fn try_recv(&mut self, side: &Side) -> Result<Cm, E>;
}

pub trait StagingEnvironment<E: Error, Sm, Cm>: CommunicatingEnvironment<Sm, Cm, E> {
    //fn are_players_ready(&self) -> bool;
    fn run (&mut self) -> Result<(), E>;
    //fn run_until<G: FnMut(&Self) -> bool> (&mut self, guard: G) -> Result<(), E>;
}

//impl<F: CardCheck<BridgeErrorStd> + Default> StagingEnvironment<BridgeErrorStd, ServerDealMessage, ClientDealMessage>
//    for ChannelDealEnvironment<F>{
impl<T > StagingEnvironment<BridgeErrorStd, ServerDealMessage, ClientDealMessage>
    for T
where T:  CommunicatingEnvironment<ServerDealMessage, ClientDealMessage, BridgeErrorStd> +
OrderGuard + WaitReady + Environment{
    /*fn are_players_ready(&self) -> bool {
        self.player_status.and(|x| *x == Ready)
    }*/

    fn run(&mut self)  -> Result<(), BridgeErrorStd>{

        if let Some(whist) = self.deal().current_side(){
            info!("Sending start signal to first player.");
            self.send(&whist, YourMove.into())?;
        }
        loop{/*
            match self.control_rx.try_recv(){
                Ok(signal) => match signal {
                    ControlCommand::Start => {
                        info!("Received 'Start'. Ignoring. Reserved for future use");
                        self.control_tx.send(ControlCommand::Start).unwrap();
                        todo!()
                    }
                    ControlCommand::Pause => {
                        info!("Received 'Pause'. Ignoring. Reserved for future use");
                    }
                    ControlCommand::Kill => {
                        info!("Received 'Kill'. Stopping world.");
                        self.send_to_all(ServerStopping.into()).unwrap_or(());
                    }
                }
                Err(e) => match e{
                    TryRecvError::Empty => {/* ignore */}
                    TryRecvError::Disconnected => {
                        warn!("Command Interface disconnected. Should not have happen, because Overseer keeps his copy of sender. Anyway sending ServerStopped to players.");
                        self.send_to_all(ServerStopping.into()).unwrap_or(());
                        return Err(BridgeError::Flow(ServerDead));
                    }
                }
            }*/
            for player in SIDES{
                match self.try_recv(&player){
                    Ok(client_message) => match client_message{
                        ClientDealMessage::Action(action) => match action{
                            DealAction::PlayCard(card) => match self.card_check(&card){
                                Ok(_) => match self.deal().current_side(){
                                    None => {
                                        warn!("Player {:?} played card when no one's turn - possibly end of deal.", player);
                                    }
                                    Some(s) if s == player || (s == player.partner() && player == self.deal().declarer())=>
                                        match self.deal_mut().insert_card(s/*self.side_correction(player)*/, card){
                                        Ok(next_side) => {
                                            info!("Player {:?} sent card {:#}, and it is accepted.", &player, &card );
                                            self.send(&player, CardAccepted(card).into()).unwrap_or(());
                                            //debug!("Side correction: {:?} -> {:?}", side, self.side_correction(side));
                                            self.send_to_all(CardPlayed(s, card).into()).unwrap_or(());
                                            if self.deal().current_trick().is_empty() {
                                                info!("Trick completed. It was {:?} so far.", self.deal().completed_tricks().len());
                                                self.send_to_all(TrickClosed(next_side).into()).unwrap_or(());
                                                if self.deal().is_completed() {
                                                    info!("Deal completed.");
                                                    self.send_to_all(DealClosed.into()).unwrap_or(());
                                                    self.send_to_all(GameOver.into()).unwrap_or(());
                                                    return Ok(());
                                                }
                                            }
                                            if self.dummy_hand().is_some(){
                                                info!("Informing next player: {:?}", &next_side);
                                                self.send(&self.next_player()/*next_side_dummy_corrected()*/.unwrap(), YourMove.into()).unwrap_or(());
                                            }
                                            else{
                                                info!("Informing dummy {:?} that it is time for him show cards.", self.deal().dummy());
                                                self.send(&self.deal().dummy(), YourMove.into()).unwrap_or(());
                                            }
                                        }
                                        Err(e) => {
                                            warn!("Player {:?} sent card {:#}. Error inserting card: {}.", player, &card, &e);
                                            self.send_to_all( ServerBridgeError(e.into()).into()).unwrap_or(());
                                        }
                                    },
                                    Some(s) => {
                                        warn!("Player {:?} sent card, when it is time to play of side {:?}", player, s)
                                    }
                                }
                                Err(e) => {
                                    warn!("Player {:?} sent card {:#} which raised error: {:?}", player, card, e);
                                    self.send_to_all(ServerBridgeError(e.clone()).into()).unwrap_or(());
                                    return Err(e);
                                }
                            }
                        }
                        ClientDealMessage::Info(info) => match info{
                            ClientDealInformation::ShowHand(hand) => {
                                if player == self.deal().dummy(){
                                    info!("Received dummy's hand: {:#}", hand);
                                    if self.dummy_hand().is_none(){
                                        info!("Setting and sending dummy's hand.");
                                        self.set_dummy_hand(hand);// = Some(hand);
                                        self.send_to_all(DummyPlacedHand(self.dummy_hand().unwrap().clone()).into()).unwrap_or(());
                                        info!("Sending player: {:?} signal 'YourMove'", &self.next_player().unwrap());
                                        self.send(&self.next_player().unwrap(), YourMove.into()).unwrap_or(());
                                    }
                                    else{
                                        warn!("Dummy's hand already set. Ignoring.");
                                    }


                                }
                            }
                        }
                        ClientDealMessage::InfoRequest(_) => {},
                        ClientDealMessage::Control(control) => match control{
                            ClientControlMessage::IamReady => {
                                info!("Player {:?} declared readiness", &player);
                                self.set_ready(&player);
                            }
                            ClientControlMessage::Quit => {
                                info!("Player {:?} has left the game. ", &player);
                                self.send_to_all(PlayerLeft(player).into()).unwrap_or(());
                                self.send_to_all(ServerStopping.into()).unwrap_or(());
                                return Err(FlowError::AbsentPlayer(player).into());
                            }
                            ClientControlMessage::ClientBridgeError(e) => {
                                warn!("Player {:?} reported error {:?}.", &player, e);
                            }
                            ClientControlMessage::NotMyTurn => {
                                warn!("Player {:?} reported it is not his turn. Possibly bad behaviour.", &player);
                            }
                        }
                    }
                    Err(try_recv_err) if try_recv_err == BridgeErrorStd::Comm(CommError::TryRecvError) => {
                        //ignore
                    }
                    Err(e) => {
                        error!("Error receiving message - probably disconnected: {:?}", e)
                    }
                }
            }

        }
    }
}