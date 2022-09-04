use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender, SendError, TryRecvError};
use crate::deal::{DealMaintainer, RegDealStd};
use crate::error::{BridgeError, BridgeErrorStd, DealError, FlowError, Mismatch};
use crate::overseer::{Overseer, PlayerStatus};
use crate::player::side::{Side, SideAssociated, SIDES};
use crate::protocol::{ClientMessage, ControlCommand, DealAction, ServerMessage};
use log::{info, warn, trace, debug};
use nom::Parser;
use parking_lot::Mutex;
use karty::figures::FigureStd;
use karty::suits::SuitStd;
use crate::distribution::hand::BridgeHand;
use crate::error::BridgeError::Flow;
use crate::error::FlowError::{MissingConnection, PlayerLeft, ServerDead};
use crate::error::TrickError::ViolatedOrder;
use crate::overseer::PlayerStatus::Ready;
use crate::player::side::Side::{East, North, South, West};
use crate::protocol::DealNotify::{CardAccepted, CardPlayed};
use crate::protocol::ServerMessage::{Deal, ServerStopping};


pub struct SimpleOverseer{
    receivers: SideAssociated<Option<Arc<Mutex<Receiver<ClientMessage>>>>>,
    senders: SideAssociated<Option<Sender<ServerMessage>>>,
    deal: RegDealStd,
    player_status: SideAssociated<PlayerStatus>,
    control_rx: Receiver<ControlCommand>,
    control_tx: Sender<ControlCommand>,
    dummy_hand: BridgeHand,

}

impl SimpleOverseer{
    pub fn new(deal: RegDealStd) -> Self{
        let (control_tx, control_rx) = channel();
        Self{
            receivers: Default::default(),
            senders: Default::default(),
            player_status: Default::default(),
            control_rx,
            deal,
            control_tx,
            dummy_hand: BridgeHand::empty(),
        }
    }

    pub fn control_sender(&self) -> Sender<ControlCommand>{
        self.control_tx.clone()
    }

    fn receiver(&self, side: &Side) -> &Arc<Mutex<Receiver<ClientMessage>>>{

        self.receivers[side].as_ref().unwrap()
    }
    fn sender(&self, side: &Side) -> &Sender<ServerMessage>{

        self.senders[side].as_ref().unwrap()
    }

    fn send_to_all(&self, message: ServerMessage) -> Result<(), BridgeErrorStd>{
        let mut err: Result<(), BridgeErrorStd> = Ok(());
        if let Some(n) = &self.senders[&North]{
            if let Err(e) = n.send(message.clone()){
                err = Err(e.into());
            }
        }
        if let Some(n) = &self.senders[&East]{
            if let Err(e) = n.send(message.clone()){
                err = Err(e.into());
            }
        }
        if let Some(n) = &self.senders[&South]{
            if let Err(e) = n.send(message.clone()){
                err = Err(e.into());
            }
        }
        if let Some(n) = &self.senders[&West]{
            if let Err(e) = n.send(message){
                err = Err(e.into());
            }
        }
        err

    }



    pub fn create_connection(&mut self, side: &Side) -> (Sender<ClientMessage>, Receiver<ServerMessage>){
        let (cms, cmr) = channel();
        let (sms, smr) = channel();
        self.senders[side]  = Some(sms);

        self.receivers[side] = Some(Arc::new(Mutex::new(cmr)));
        //println!("Connection: {:?}, {:?}", &side, &self.senders[&side]);
        //println!("Connection: {:?}, {:?}", &side, &self.receivers[&side]);
        (cms, smr)
    }
    pub fn deal(&self) -> &RegDealStd{
        &self.deal
    }

    pub fn are_players_ready(&self) -> bool{
        /*self.player_status[South] == Ready && self.player_status[North] == Ready &&
            self.player_status[]

         */
        self.player_status.and(|x| *x== Ready)
    }
    /// Waits for readiness, this works in round robin try_recv on channels
    /// ```
    ///
    /// use bridge_core::cards::trump::Trump;
    /// use bridge_core::deal::{Contract, RegDealStd};
    /// use bridge_core::overseer::SimpleOverseer;
    /// use bridge_core::karty::suits::SuitStd::Spades;
    /// use bridge_core::player::side::Side;
    /// use bridge_core::bidding::Bid;
    /// use std::thread;
    /// use bridge_core::player::side::Side::{East, North, South, West};
    /// use bridge_core::protocol::ClientMessage;
    /// let deal = RegDealStd::new(Contract::new(Side::East, Bid::init(Trump::Colored(Spades), 2).unwrap()));
    /// let mut simple_overseer = SimpleOverseer::new(deal);
    /// let (n_tx, n_rx) = simple_overseer.create_connection(&North);
    /// let (s_tx, s_rx) = simple_overseer.create_connection(&South);
    /// let (e_tx, e_rx) = simple_overseer.create_connection(&East);
    /// let (w_tx, w_rx) = simple_overseer.create_connection(&West);
    /// assert!(!simple_overseer.are_players_ready());
    /// thread::scope(|s|{
    ///     s.spawn(||{
    ///        simple_overseer.wait_for_readiness_rr();
    ///     });
    ///     n_tx.send(ClientMessage::Ready);
    ///     s_tx.send(ClientMessage::Ready);
    ///     e_tx.send(ClientMessage::Ready);
    ///     w_tx.send(ClientMessage::Ready);
    ///
    /// });
    /// assert!(simple_overseer.are_players_ready());
    ///
    /// ```
    pub fn wait_for_readiness_rr(&mut self) -> Result<(), BridgeErrorStd>{
        //let mg_north = self.receivers[&North].unwrap().as_ref().lock();

        /*if self.receivers.or(|x| x.is_none()){
            return BridgeErrorStd::Flow(MissingConnection())
        }*/
        if let Some(undefined) = self.receivers.find(|x| x.is_none()){
            return Err(BridgeError::Flow(MissingConnection(undefined)))
        }
        if let Some(undefined) = self.senders.find(|x| x.is_none()){
            return Err(BridgeError::Flow(MissingConnection(undefined)))
        }

        let north_guard = self.receivers[&North].as_ref().unwrap().as_ref().lock();
        let east_guard = self.receivers[&East].as_ref().unwrap().as_ref().lock();
        let south_guard = self.receivers[&South].as_ref().unwrap().as_ref().lock();
        let west_guard = self.receivers[&West].as_ref().unwrap().as_ref().lock();

        let receiver_guards = SideAssociated::new(north_guard, east_guard, south_guard, west_guard);

        while !self.are_players_ready(){
            for side in SIDES{
                match receiver_guards[&side].try_recv(){
                    Ok(m) => match m{

                        ClientMessage::Ready => {
                            info!("Player {:?} declared to be ready during asking for readiness.", &side);
                            self.player_status[&side] = Ready
                        }
                        ClientMessage::Quit => {
                            info!("Player {:?} sent 'Quit' signal during asking for readiness.", &side);
                            return Err(PlayerLeft(side).into())
                        }
                        ClientMessage::Dealing(_) => {
                            info!("Player {:?} made dealing action, during readiness check.", &side);
                            self.senders[&side].as_ref().unwrap().send(ServerMessage::ServerNotReady)?;
                        }
                        ClientMessage::Bidding(_) => {
                            info!("Player {:?} made a bid during dealing phase (readiness check). Response not implemented.", &side);
                        }
                        ClientMessage::DealInfo(_) => {
                            info!("Player {:?} requested info on deal. Response not implemented.", &side);
                        }
                        ClientMessage::BiddingInfo(_) => {
                            info!("Player {:?} requested info on bidding. Response not implemented. Such request is not expected.", &side);
                        }
                        ClientMessage::Error(e) => {
                            warn!("Player {:?} sent error message: {:?}.", &side, e)
                        }
                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {/*just skip*/}
                        TryRecvError::Disconnected => {
                            info!("Player {:?} disconnected during waiting for readiness (broken channel).", &side);
                            return Err(PlayerLeft(side).into())
                        }
                    }
                }
            }
        }
        Ok(())

    }
    pub fn oversee_deal_rr(&mut self) -> Result<(), BridgeErrorStd>{
        self.wait_for_readiness_rr()?;
        let receiver_guards = SideAssociated{
            north: self.receivers[&North].as_ref().unwrap().as_ref().lock(),
            east: self.receivers[&East].as_ref().unwrap().as_ref().lock(),
            south: self.receivers[&South].as_ref().unwrap().as_ref().lock(),
            west: self.receivers[&West].as_ref().unwrap().as_ref().lock(),
        };
        let sender_guards = SideAssociated{
            north: self.senders[&North].as_ref().unwrap(),
            east: self.senders[&East].as_ref().unwrap(),
            south: self.senders[&South].as_ref().unwrap(),
            west: self.senders[&West].as_ref().unwrap()
        };

        while !self.deal.is_completed(){
            match self.control_rx.try_recv(){
                Ok(signal) => match signal {
                    ControlCommand::Start => {
                        info!("Received 'Start'. Ignoring. Reserved for future use");
                    }
                    ControlCommand::Pause => {
                        info!("Received 'Pause'. Ignoring. Reserved for future use");
                    }
                    ControlCommand::Kill => {
                        info!("Received 'Kill'. Stopping overseer.");
                        self.send_to_all(ServerStopping).unwrap_or(());
                    }
                }
                Err(e) => match e{
                    TryRecvError::Empty => {/* ignore */}
                    TryRecvError::Disconnected => {
                        warn!("Command Interface disconnected. Should not have happen, because Overseer keeps his copy of sender. Anyway sending ServerStopped to players.");
                        self.send_to_all(ServerStopping).unwrap_or(());
                        return Err(BridgeError::Flow(ServerDead));
                    }
                }
            }
            for side in SIDES{
                match receiver_guards[&side].try_recv(){
                    Ok(message) => match message{
                        ClientMessage::Dealing(action) => match action {
                            DealAction::PlayCard(card) => match self.deal.current_side() {
                                None => {
                                    info!("Player {:?} sent card when no one's turn, game should have ended.", side);

                                }
                                Some(s) => {
                                    if self.deal.current_side() == side || (self.deal.declarer() == side && self.deal.current_side() == side.partner()){
                                        match self.deal.insert_card(s, card){
                                            Ok(next_side) => {
                                                info!("Player {:?} sent card {:#}", &s, &card );
                                                sender_guards[&side].send(Deal(CardAccepted(card))).unwrap_or(());
                                                self.send_to_all(ServerMessage::Deal(CardPlayed(s, card))).unwrap_or(());
                                                //sender_guards[&next_side].
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                    else{

                                    }
                                }
                            },



                            DealAction::NotMyTurn => {}
                            DealAction::ShowHand(_) => {}
                        }
                        ClientMessage::Bidding(_) => {/*ignoring */}
                        ClientMessage::DealInfo(dir) => {
                            info!("Player {:?} requested game related information: {:?}. Request ignored as not yet implemented", &side, dir);
                        }
                        ClientMessage::BiddingInfo(_) => {/*ignoring */}
                        ClientMessage::Error(e) => match e{
                            _ => {
                                warn!("Player {:?} reported error {:?}.", &side, e);
                                todo!();
                            }
                        }
                        ClientMessage::Ready => {
                            info!("Player {:?} sent signal 'Ready'", &side);
                            self.player_status[&side] = Ready; //or whatever they should be already ready
                        }
                        ClientMessage::Quit => {
                            warn!("Player {:?} has left the game. ", &side);
                            self.send_to_all(ServerMessage::PlayerLeft(side)).unwrap_or(());
                            self.send_to_all(ServerMessage::ServerStopping).unwrap_or(());
                            return Err(FlowError::PlayerLeft(side).into());
                        }
                    }
                    Err(e) => match e{
                        TryRecvError::Empty => {/* ignore */}
                        TryRecvError::Disconnected => {
                            info!("Player {:?} disconnected. Closing game.", &side);
                            self.send_to_all(ServerStopping).unwrap_or(());
                            return Err(BridgeError::Flow(PlayerLeft(side)));
                        }
                    }
                }
            }
        }
        todo!()
    }


}

impl Overseer for SimpleOverseer{
    fn run(&mut self) -> Result<(), BridgeErrorStd> {
        self.oversee_deal_rr()


    }
}


