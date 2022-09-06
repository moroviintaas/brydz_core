use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use log::{error, info, warn};
use karty::cards::CardStd;
use crate::deal::{DealMaintainer, RegDealStd};
use crate::distribution::hand::BridgeHand;
use crate::error::{BridgeError, BridgeErrorStd, FlowError};
use crate::error::FlowError::{MissingConnection, ServerDead};
use crate::player::side::{Side, SideAssociated, SIDES};
use crate::protocol::{ClientControlMessage, ClientDealInformation, ClientDealMessage, ControlCommand, DealAction, ServerDealMessage};
use crate::protocol::DealNotify::{CardAccepted, CardPlayed, DealClosed, DummyPlacedHand, TrickClosed, YourMove};
use crate::protocol::ServerControlMessage::{GameOver, PlayerLeft, ServerBridgeError, ServerStopping};
use crate::world::environment::{ CardCheck, CommunicatingEnvironment, OrderGuard, StagingEnvironment};
use crate::world::PlayerStatus;
use crate::world::PlayerStatus::Ready;

#[derive(Copy, Clone, Default, Debug)]
pub struct NoCardCheck{}

impl CardCheck<BridgeErrorStd> for NoCardCheck{
    fn check(&self, _: &CardStd) -> Result<(), BridgeErrorStd> {
        Ok(())
    }
}

pub struct ChannelDealEnvironment<F: CardCheck<BridgeErrorStd> + Default>{
    receivers: SideAssociated<Option<Receiver<ClientDealMessage>>>,
    senders: SideAssociated<Option<Sender<ServerDealMessage>>>,
    deal: RegDealStd,
    player_status: SideAssociated<PlayerStatus>,
    control_rx: Receiver<ControlCommand>,
    control_tx: Sender<ControlCommand>,
    dummy_hand: Option<BridgeHand>,
    checker: F
}

impl<F: CardCheck<BridgeErrorStd> + Default> ChannelDealEnvironment<F>{
    pub fn new(deal: RegDealStd) -> Self{
        let (control_tx, control_rx) = channel();
        Self{
            receivers: Default::default(),
            senders: Default::default(),
            player_status: Default::default(),
            control_rx,
            deal,
            control_tx,
            dummy_hand: None,//BridgeHand::empty(),
            checker: F::default()
        }
    }

    fn next_side_dummy_corrected(&self) -> Option<Side>{
        match self.deal.current_side(){
            None => None,
            Some(s) if s == self.deal.dummy() => Some(s.partner()),
            Some(s) => Some(s)
        }
    }
    pub fn create_connection(&mut self, side: &Side) -> (Sender<ClientDealMessage>, Receiver<ServerDealMessage>){
        let (cms, cmr) = channel();
        let (sms, smr) = channel();
        self.senders[side]  = Some(sms);

        self.receivers[side] = Some(cmr);
        (cms, smr)
    }
    fn side_correction(&self, side: Side) -> Side{
        if side == self.deal.declarer() && self.deal.current_side().unwrap_or(side) == self.deal.dummy(){
            return side.partner();
        }
        side
    }
}

impl<F: CardCheck<BridgeErrorStd> + Default> CommunicatingEnvironment<ServerDealMessage, ClientDealMessage, BridgeErrorStd> for ChannelDealEnvironment<F>{
    fn send(&self, side: &Side, message: ServerDealMessage) -> Result<(), BridgeErrorStd> {
        match &self.senders[side]{
            None => Err(MissingConnection(*side).into()),
            Some(sender) => {
                sender.send(message).map_err(|e| e.into())
            }
        }
    }

    fn send_to_all(&self, message: ServerDealMessage) -> Result<(), BridgeErrorStd> {
        let mut result: Result<(), BridgeErrorStd> = Ok(());
        for side in SIDES{
            match &self.senders[&side]{
                None => {
                    result = Err(MissingConnection(side).into());
                },
                Some(sender) => {
                    if let Err(e) = sender.send(message.clone()){
                        result = Err(e.into());
                    }
                }
            }

        }
        result
    }

    fn recv(&self, side: &Side) -> Result<ClientDealMessage, BridgeErrorStd> {
        match &self.receivers[side]{
            None => Err(MissingConnection(*side).into()),
            Some(receiver) => {
                receiver.recv().map_err(|e| e.into())
            }
        }
    }

    fn try_recv(&self, side: &Side) -> Result<ClientDealMessage, BridgeErrorStd> {
        match &self.receivers[side]{
            None => Err(MissingConnection(*side).into()),
            Some(receiver) => {
                receiver.try_recv().map_err(|e| e.into())
            }
        }
    }
}

impl<F: CardCheck<BridgeErrorStd> + Default> OrderGuard for ChannelDealEnvironment<F>{
    fn current_side(&self) -> Option<Side> {
        self.deal.current_side()
    }

    fn is_dummy_placed(&self) -> bool {
        self.dummy_hand.is_some()
    }
}

impl<F: CardCheck<BridgeErrorStd> + Default> StagingEnvironment<BridgeErrorStd, ServerDealMessage, ClientDealMessage>
    for ChannelDealEnvironment<F>{
    fn are_players_ready(&self) -> bool {
        self.player_status.and(|x| *x == Ready)
    }

    fn run(&mut self)  -> Result<(), BridgeErrorStd>{

        if let Some(whist) = self.deal.current_side(){
            info!("Sending start signal to first player.");
            self.send(&whist, YourMove.into())?;
        }
        loop{
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
            }
            for player in SIDES{
                match self.try_recv(&player){
                    Ok(client_message) => match client_message{
                        ClientDealMessage::Action(action) => match action{
                            DealAction::PlayCard(card) => match self.checker.check(&card){
                                Ok(_) => match self.deal.current_side(){
                                    None => {
                                        warn!("Player {:?} played card when no one's turn - possibly end of deal.", player);
                                    }
                                    Some(s) if s == player || (s == player.partner() && player == self.deal.declarer())=> match self.deal.insert_card(self.side_correction(player), card){
                                        Ok(next_side) => {
                                            info!("Player {:?} sent card {:#}, and it is accepted.", &player, &card );
                                            self.send(&player, CardAccepted(card).into()).unwrap_or(());
                                            //debug!("Side correction: {:?} -> {:?}", side, self.side_correction(side));
                                            self.send_to_all(CardPlayed(s, card).into()).unwrap_or(());
                                            if self.deal.current_trick().is_empty() {
                                                info!("Trick completed. It was {:?} so far.", self.deal.completed_tricks().len());
                                                self.send_to_all(TrickClosed(next_side).into()).unwrap_or(());
                                                if self.deal.is_completed() {
                                                    info!("Deal completed.");
                                                    self.send_to_all(DealClosed.into()).unwrap_or(());
                                                    self.send_to_all(GameOver.into()).unwrap_or(());
                                                    return Ok(());
                                                }
                                            }
                                            if self.dummy_hand.is_some(){
                                                info!("Informing next player: {:?}", &next_side);
                                                self.send(&self.next_side_dummy_corrected().unwrap(), YourMove.into()).unwrap_or(());
                                            }
                                            else{
                                                info!("Informing dummy {:?} that it is time for him show cards.", self.deal.dummy());
                                                self.send(&self.deal.dummy(), YourMove.into()).unwrap_or(());
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
                                if player == self.deal.dummy(){
                                    info!("Received dummy's hand: {:#}", hand);
                                    if self.dummy_hand.is_none(){
                                        info!("Setting and sending dummy's hand.");
                                        self.dummy_hand = Some(hand);
                                        self.send_to_all(DummyPlacedHand(self.dummy_hand.as_ref().unwrap().clone()).into()).unwrap_or(());
                                        info!("Sending player: {:?} signal 'YourMove'", &self.next_side_dummy_corrected().unwrap());
                                        self.send(&self.next_side_dummy_corrected().unwrap(), YourMove.into()).unwrap_or(());
                                    }
                                    else{
                                        warn!("Dummy's hand already set. Ignoring.");
                                    }


                                }
                            }
                        }
                        ClientDealMessage::InfoRequest(_) => {}
                        ClientDealMessage::Control(control) => match control{
                            ClientControlMessage::IamReady => {
                                info!("Player {:?} declared readiness", &player);
                                self.player_status[&player] = Ready;
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
                    Err(try_recv_err) if try_recv_err == BridgeErrorStd::Flow(FlowError::TryRecvError) => {
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
