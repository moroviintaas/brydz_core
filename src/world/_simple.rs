use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use crate::deal::{DealMaintainer, RegDealStd};
use crate::error::{BridgeError, BridgeErrorStd,  FlowError};
use crate::world::{Overseer, PlayerStatus};
use crate::player::side::{Side, SideAssociated, SIDES};
use crate::protocol::{ClientControlMessage, ClientDealInformation, ClientDealMessage,  ControlCommand, DealAction, DealNotify,  ServerDealMessage,};
use log::{debug, info, warn};
use parking_lot::Mutex;
use crate::distribution::hand::BridgeHand;
use crate::error::FlowError::{MissingConnection, AbsentPlayer, ServerDead};
use crate::world::PlayerStatus::Ready;
use crate::player::side::Side::{East, North, South, West};
use crate::protocol::DealNotify::{CardAccepted, CardPlayed, DealClosed, DummyPlacedHand, TrickClosed, YourMove};
use crate::protocol::ServerControlMessage::{GameOver, PlayerLeft, ServerBridgeError,  ServerStopping};


pub struct SimpleOverseer{
    receivers: SideAssociated<Option<Arc<Mutex<Receiver<ClientDealMessage>>>>>,
    senders: SideAssociated<Option<Sender<ServerDealMessage>>>,
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


    fn send_to_all(&self, message: ServerDealMessage) -> Result<(), BridgeErrorStd>{
        let mut err: Result<(), BridgeErrorStd> = Ok(());
        if let Some(n) = &self.senders[&North]{
            if let Err(e) = n.send(message.clone()){
                warn!("Error sending {:?} to North.", message);
                err = Err(e.into());
            }
        }
        if let Some(n) = &self.senders[&East]{
            if let Err(e) = n.send(message.clone()){
                warn!("Error sending {:?} to East.", message);
                err = Err(e.into());
            }
        }
        if let Some(n) = &self.senders[&South]{
            if let Err(e) = n.send(message.clone()){
                warn!("Error sending {:?} to South.", message);
                err = Err(e.into());
            }
        }
        if let Some(n) = &self.senders[&West]{
            if let Err(e) = n.send(message.clone()){
                warn!("Error sending {:?} to West.", message);
                err = Err(e.into());
            }
        }
        err

    }



    pub fn create_connection(&mut self, side: &Side) -> (Sender<ClientDealMessage>, Receiver<ServerDealMessage>){
        let (cms, cmr) = channel();
        let (sms, smr) = channel();
        self.senders[side]  = Some(sms);

        self.receivers[side] = Some(Arc::new(Mutex::new(cmr)));
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

    fn next_side_dummy_corrected(&self) -> Option<Side>{
        match self.deal.current_side(){
            None => None,
            Some(s) if s == self.deal.dummy() => Some(s.partner()),
            Some(s) => Some(s)
        }
    }
    /// Waits for readiness, this works in round robin try_recv on channels
    /// ```
    ///
    /// use brydz_core::cards::trump::Trump;
    /// use brydz_core::deal::{Contract, RegDealStd};
    /// use brydz_core::world::SimpleOverseer;
    /// use brydz_core::karty::suits::SuitStd::Spades;
    /// use brydz_core::player::side::Side;
    /// use brydz_core::bidding::Bid;
    /// use std::thread;
    /// use brydz_core::player::side::Side::{East, North, South, West};
    /// use brydz_core::protocol::ClientControlMessage::IamReady;
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
    ///     n_tx.send(IamReady.into());
    ///     s_tx.send(IamReady.into());
    ///     e_tx.send(IamReady.into());
    ///     w_tx.send(IamReady.into());
    ///
    /// });
    /// assert!(simple_overseer.are_players_ready());
    ///
    /// ```
    pub fn wait_for_readiness_rr(&mut self) -> Result<(), BridgeErrorStd>{
        //let mg_north = self.receivers[&North].unwrap().as_ref().lock();

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
                        ClientDealMessage::Control(control) => match control{
                            ClientControlMessage::IamReady => {
                                info!("Player {:?} declared to be ready during asking for readiness.", &side);
                                self.player_status[&side] = Ready
                            }
                            ClientControlMessage::Quit => {
                                info!("Player {:?} sent 'Quit' signal during asking for readiness.", &side);
                                return Err(AbsentPlayer(side).into())
                            }
                            ClientControlMessage::ClientBridgeError(e) => {
                                warn!("Player {:?} reported error: {:?}.", &side, e);
                            }
                            m => {
                                debug!("Player {:?} sent control message: {:?}", side, m);
                            }

                        },
                        ClientDealMessage::Action(deal_message) =>  {
                            warn!("Player {:?} attempted to perform action: {:?}", &side, deal_message);
                        },
                        ClientDealMessage::Info(info) => {
                            debug!("Player {:?} sent info: {:?}", &side, info);
                        },
                        ClientDealMessage::InfoRequest(info_request) => {
                            debug!("Player {:?} requested info: {:?}", &side, info_request);
                        },

                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {/*just skip*/}
                        TryRecvError::Disconnected => {
                            info!("Player {:?} disconnected during waiting for readiness (broken channel).", &side);
                            return Err(AbsentPlayer(side).into())
                        }
                    }
                }
            }
        }
        Ok(())

    }

    fn wait_for_first_card(&mut self) -> Result<(), BridgeErrorStd>{
        let whist = self.deal.declarer().next();
        let receiver_guard = self.receivers[&whist].as_ref().unwrap().as_ref().lock();
        let sender = self.senders[&whist].as_ref().unwrap();

        info!("Asking first defender to start playing.");
        sender.send(YourMove.into()).unwrap_or(());
        let declarer = self.deal.declarer();

        loop{ //waiting for first card
            match receiver_guard.recv(){
                 Err(_) => {

                    warn!("Failed receiving first card from first defender ({:?}). Ending game.", &whist);
                    self.send_to_all(PlayerLeft(declarer).into()).unwrap_or(());
                    self.send_to_all(ServerStopping.into()).unwrap_or(());
                    return Err(FlowError::RecvError.into());
                },
                Ok(client_message) => match client_message{
                    ClientDealMessage::Action(action) => match action{
                        DealAction::PlayCard(c) => match self.deal.insert_card(whist, c) {
                            Ok(_) => {
                                info!("Received card {:#} from declarer ({:?})", c, &whist);
                                sender.send(CardAccepted(c).into())?;
                                self.send_to_all(CardPlayed(whist, c).into()).unwrap_or(());

                                return Ok(())
                            }
                            Err(e) => {
                                warn!("Received card from first defender, failed to add do deal however: {}", e);
                            }
                        }
                    }
                    ClientDealMessage::InfoRequest(info_request) => {
                        debug!("Player {:?} requested info: {:?}.", whist, info_request);
                    }
                    ClientDealMessage::Info(info) => {
                        debug!("Player {:?} sent info: {:?}.", whist, info);
                    }
                    ClientDealMessage::Control(control) => match control{
                        ClientControlMessage::IamReady => {}
                        ClientControlMessage::Quit => {
                            warn!("Failed receiving first card from first defender ({:?}). Ending game.", &whist);
                            self.send_to_all(PlayerLeft(declarer).into()).unwrap_or(());
                            self.send_to_all(ServerStopping.into()).unwrap_or(());
                            return Err(FlowError::RecvError.into());
                        }
                        ClientControlMessage::ClientBridgeError(e) => {
                            warn!("During waiting wor first card player {:?} reported error {:?}.", &whist, e);
                        }
                        ClientControlMessage::NotMyTurn => {}
                    }
                }


            }
        }

    }
    fn wait_for_dummy_hand(&mut self) -> Result<(), BridgeErrorStd>{
        let dummy = self.deal.dummy();
        let receiver_guard = self.receivers[&dummy].as_ref().unwrap().as_ref().lock();
        let sender = self.senders[&dummy].as_ref().unwrap();

        info!("Asking dummy to show hand.");
        sender.send(YourMove.into()).unwrap_or(());
        let declarer = self.deal.declarer();
        loop{ //waiting for first card
            match receiver_guard.recv(){
                 Err(_) => {

                    warn!("Failed receiving first hand from dummy ({:?}). Ending game.", &dummy);
                    self.send_to_all(PlayerLeft(declarer).into()).unwrap_or(());
                    self.send_to_all(ServerStopping.into()).unwrap_or(());
                    return Err(FlowError::RecvError.into());
                },
                Ok(deal_message) => match deal_message{
                    ClientDealMessage::Action(action) => {
                        warn!("Dummy ({:?}) attempted to perform action: {:?}.", dummy, action);
                    }
                    ClientDealMessage::Info(information) => match information{
                        ClientDealInformation::ShowHand(hand) => {
                            info!("Dummy ({:?}) sent hand: {:?}", dummy, &hand);
                            self.dummy_hand = hand;
                            //println!("{:?}", self.dummy_hand.clone());
                            self.send_to_all(DummyPlacedHand(self.dummy_hand.clone()).into()).unwrap_or(());
                            return Ok(());
                        }
                    }
                    ClientDealMessage::InfoRequest(_) => {}
                    ClientDealMessage::Control(_) => {}
                }


            }
        }
    }
    fn oversee_rest_deal_rr(&mut self) -> Result<(), BridgeErrorStd>{
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
        info!("Notifying player {:?} (declarer) to play card (dummy's)", self.next_side_dummy_corrected().unwrap());
        sender_guards[&self.next_side_dummy_corrected().unwrap()].send(YourMove.into()).unwrap_or(());
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

            for side in SIDES{
                match receiver_guards[&side].try_recv(){
                    Ok(message) => match message{

                        ClientDealMessage::Action(action) => match action{
                            DealAction::PlayCard(card) => match self.deal.current_side() {
                                None => {
                                    warn!("Player {:?} played card when no one's turn - possibly end of deal.", side);
                                }
                                Some(s) => {
                                    if s == side || (self.deal.declarer() == side && s == side.partner()) {
                                        match self.deal.insert_card(s, card) {
                                            Ok(next_side) => {
                                                info!("Player {:?} sent card {:#}, and it is accepted.", &s, &card );
                                                sender_guards[&side].send(CardAccepted(card).into()).unwrap_or(());
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
                                                info!("Informing next player: {:?}", &next_side);
                                                //sender_guards[&next_side].send(ServerMessage::Deal(YourMove)).unwrap_or(());
                                                sender_guards[&self.next_side_dummy_corrected().unwrap()].send(DealNotify::YourMove.into()).unwrap_or(());
                                            }
                                            Err(e) => {
                                                warn!("Player {:?} sent card {:#}. Error inserting card: {}.", side, &card, &e);
                                                sender_guards[&side].send(ServerBridgeError(e.into()).into()).unwrap_or(());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        ClientDealMessage::Info(info) => {
                            debug!("Player {:?} sent info: {:?}. Doing nothing with it for now.", side ,info );
                        }
                        ClientDealMessage::InfoRequest(info_request) => {
                            debug!("Player {:?} sent info request: {:?}.", side, info_request);
                        }
                        ClientDealMessage::Control(control) => match control{
                            ClientControlMessage::IamReady => {}
                            ClientControlMessage::Quit => {
                                info!("Player {:?} has left the game. ", &side);
                                self.send_to_all(PlayerLeft(side).into()).unwrap_or(());
                                self.send_to_all(ServerStopping.into()).unwrap_or(());
                                return Err(FlowError::AbsentPlayer(side).into());
                            }
                            ClientControlMessage::ClientBridgeError(e) => {
                                warn!("Player {:?} reported error {:?}.", &side, e);
                            }
                            ClientControlMessage::NotMyTurn => {
                                warn!("Player {:?} reported it is not his turn. Possibly bad behaviour.", side);
                            }

                        }
                    }
                    Err(e) => match e{
                        TryRecvError::Empty => {/* ignore */}
                        TryRecvError::Disconnected => {
                            info!("Player {:?} disconnected. Closing game.", &side);
                            self.send_to_all(ServerStopping.into()).unwrap_or(());
                            return Err(BridgeError::Flow(AbsentPlayer(side)));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn oversee_deal_rr(&mut self) -> Result<(), BridgeErrorStd>{


        self.wait_for_readiness_rr()?;
        self.wait_for_first_card()?;
        self.wait_for_dummy_hand()?;
        self.oversee_rest_deal_rr()






    }


}

impl Overseer for SimpleOverseer{
    fn run(&mut self) -> Result<(), BridgeErrorStd> {
        self.oversee_deal_rr()


    }
}


