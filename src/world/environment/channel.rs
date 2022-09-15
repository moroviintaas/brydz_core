use std::sync::mpsc::{channel, Receiver, Sender};
use karty::cards::CardStd;
use crate::deal::{DealMaintainer, RegDealStd};
use crate::distribution::hand::BridgeHand;
use crate::error::{ BridgeErrorStd,};
use crate::error::FlowError::{MissingConnection};
use crate::player::side::{Side, SideAssociated, SIDES};
use crate::protocol::{ ClientDealMessage, ControlCommand, ServerDealMessage};
use crate::world::environment::{CardCheck, CommunicatingEnvironment, Environment, OrderGuard,  WaitReady};
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
    #[allow(dead_code)]
    control_rx: Receiver<ControlCommand>,
    #[allow(dead_code)]
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
    /*
    fn next_side_dummy_corrected(&self) -> Option<Side>{
        match self.deal.current_side(){
            None => None,
            Some(s) if s == self.deal.dummy() => Some(s.partner()),
            Some(s) => Some(s)
        }
    }

     */
    pub fn create_connection(&mut self, side: &Side) -> (Sender<ClientDealMessage>, Receiver<ServerDealMessage>){
        let (cms, cmr) = channel();
        let (sms, smr) = channel();
        self.senders[side]  = Some(sms);

        self.receivers[side] = Some(cmr);
        (cms, smr)
    }
    /*
    fn side_correction(&self, side: Side) -> Side{
        if side == self.deal.declarer() && self.deal.current_side().unwrap_or(side) == self.deal.dummy(){
            return side.partner();
        }
        side
    }*/
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

    fn recv(&mut self, side: &Side) -> Result<ClientDealMessage, BridgeErrorStd> {
        match &self.receivers[side]{
            None => Err(MissingConnection(*side).into()),
            Some(receiver) => {
                receiver.recv().map_err(|e| e.into())
            }
        }
    }

    fn try_recv(&mut self, side: &Side) -> Result<ClientDealMessage, BridgeErrorStd> {
        match &self.receivers[side]{
            None => Err(MissingConnection(*side).into()),
            Some(receiver) => {
                receiver.try_recv().map_err(|e| e.into())
            }
        }
    }
}

impl<F: CardCheck<BridgeErrorStd> + Default> WaitReady for  ChannelDealEnvironment<F>{
    fn are_players_ready(&self) -> bool {
        self.player_status.and(|x| *x == Ready)
    }

    fn set_ready(&mut self, side: &Side) {
        self.player_status[side] = Ready
    }
}

impl<F: CardCheck<BridgeErrorStd> + Default> Environment for  ChannelDealEnvironment<F>{
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

impl<F: CardCheck<BridgeErrorStd> + Default> OrderGuard for ChannelDealEnvironment<F>{
    fn current_side(&self) -> Option<Side> {
        self.deal.current_side()
    }

    fn is_dummy_placed(&self) -> bool {
        self.dummy_hand.is_some()
    }
}


