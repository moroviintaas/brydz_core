use log::{error, info, warn};
use karty::cards::CardStd;
use crate::deal::{DealMaintainer, RegDealStd};
use crate::distribution::hand::BridgeHand;
use crate::error::{BridgeErrorStd, CommError, FlowError};
use crate::player::side::{Side, SideAssociated, SIDES};
use crate::protocol::{ClientControlMessage, ClientDealInformation, ClientDealMessage, DealAction, ServerDealMessage};
use crate::protocol::DealNotify::{CardAccepted, CardPlayed, DealClosed, DummyPlacedHand, ShowYourHand, TrickClosed, YourMove};
use crate::protocol::ServerControlMessage::{GameOver, PlayerLeft, ServerBridgeError, ServerStopping};
use crate::world::comm::CommunicationEnd;
use crate::world::environment::{AutomaticEnvironment, CardCheck, CommunicatingEnvironment, Environment, OrderGuard, WaitReady};
use crate::world::PlayerStatus;
use crate::world::PlayerStatus::Ready;

pub struct RoundRobinDealEnvironment<Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>>{
    comms: SideAssociated<Comm>,
    player_status: SideAssociated<PlayerStatus>,
    dummy_hand: Option<BridgeHand>,
    checker: C,
    deal: RegDealStd,
}

impl<Comm, C> RoundRobinDealEnvironment<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>{

    pub fn new(comms: SideAssociated<Comm>, deal: RegDealStd, checker: C) -> Self{
        Self{comms, deal, checker, dummy_hand: None, player_status: SideAssociated::default()}
    }

    pub fn run(&mut self) -> Result<(), BridgeErrorStd>{
        if let Some(whist) = self.deal().current_side(){
            info!("Sending start signal to first player.");
            self.send(&whist, YourMove.into())?;
        }

        loop{
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
                                                self.send(&self.deal().dummy(), ShowYourHand.into()).unwrap_or(());
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

/*
impl<Comm, C> RoundRobinEnvironmentStd<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>,
Self: Environment + WaitReady + CommunicatingEnvironment<ServerDealMessage, ClientDealMessage, BridgeErrorStd>{

*/

impl<Comm, C> OrderGuard for RoundRobinDealEnvironment<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>{
    fn current_side(&self) -> Option<Side> {
        self.deal.current_side()
    }

    fn is_dummy_placed(&self) -> bool {
        self.dummy_hand.is_some()
    }

}
impl<Comm, C> WaitReady for RoundRobinDealEnvironment<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>,
Self: CommunicatingEnvironment<ServerDealMessage, ClientDealMessage, BridgeErrorStd>
{
    fn are_players_ready(&self) -> bool {
        self.player_status.and(|x| *x == Ready)
    }

    fn set_ready(&mut self, side: &Side) {
        self.player_status[side] = Ready
    }
}





impl<Comm, C> Environment for RoundRobinDealEnvironment<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>{
    fn deal(&self) -> &RegDealStd {
        &self.deal
    }

    fn deal_mut(&mut self) -> &mut RegDealStd {
        &mut self.deal
    }

    fn card_check(&self, card: &CardStd) -> Result<(), BridgeErrorStd> {
        self.checker.check(card)
    }

    fn dummy_hand(&self) -> Option<&BridgeHand> {
        self.dummy_hand.as_ref()
    }

    fn set_dummy_hand(&mut self, hand: BridgeHand) {
        self.dummy_hand = Some(hand)
    }

    fn next_player(&self) -> Option<Side> {
        match self.deal().current_side(){
            None => None,
            Some(dummy) if dummy == self.deal().dummy() => Some(dummy.partner()),
            Some(n) => Some(n)
        }
    }
}

impl<Comm, C> CommunicatingEnvironment<ServerDealMessage, ClientDealMessage, BridgeErrorStd> for RoundRobinDealEnvironment<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>{
    fn send(&self, side: &Side, message: ServerDealMessage) -> Result<(), BridgeErrorStd> {
        self.comms[side].send(message)
    }

    fn send_to_all(&self, message: ServerDealMessage) -> Result<(), BridgeErrorStd> {
        let mut result: Result<(), BridgeErrorStd> = Ok(());
        for side in SIDES{
            if let Err(e) = self.comms[&side].send(message.clone()){
                result = Err(e)
            }
        }
        result
    }

    fn recv(&mut self, side: &Side) -> Result<ClientDealMessage, BridgeErrorStd> {
        self.comms[side].recv()
    }

    fn try_recv(&mut self, side: &Side) -> Result<ClientDealMessage, BridgeErrorStd> {
        self.comms[side].try_recv()
    }


}
impl<Comm, C> AutomaticEnvironment for RoundRobinDealEnvironment<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>{
    fn run_auto(&mut self) -> Result<(), BridgeErrorStd> {
        self.run()
    }
}