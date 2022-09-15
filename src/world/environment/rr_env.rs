use karty::cards::CardStd;
use crate::deal::{DealMaintainer, RegDealStd};
use crate::distribution::hand::BridgeHand;
use crate::error::BridgeErrorStd;
use crate::player::side::{Side, SideAssociated, SIDES};
use crate::protocol::{ClientDealMessage, ServerDealMessage};
use crate::world::comm::CommunicationEnd;
use crate::world::environment::{CardCheck, CommunicatingEnvironment, Environment, OrderGuard, WaitReady};
use crate::world::PlayerStatus;
use crate::world::PlayerStatus::Ready;

pub struct RoundRobinEnvironmentStd<Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>>{
    comms: SideAssociated<Comm>,
    player_status: SideAssociated<PlayerStatus>,
    dummy_hand: Option<BridgeHand>,
    checker: C,
    deal: RegDealStd,
}

impl<Comm, C> RoundRobinEnvironmentStd<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>{

    pub fn new(comms: SideAssociated<Comm>, deal: RegDealStd, checker: C) -> Self{
        Self{comms, deal, checker, dummy_hand: None, player_status: SideAssociated::default()}
    }
}

/*
impl<Comm, C> RoundRobinEnvironmentStd<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>,
Self: Environment + WaitReady + CommunicatingEnvironment<ServerDealMessage, ClientDealMessage, BridgeErrorStd>{

*/

impl<Comm, C> OrderGuard for RoundRobinEnvironmentStd<Comm, C>
where Comm: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>, C: CardCheck<BridgeErrorStd>{
    fn current_side(&self) -> Option<Side> {
        self.deal.current_side()
    }

    fn is_dummy_placed(&self) -> bool {
        self.dummy_hand.is_some()
    }

}
impl<Comm, C> WaitReady for RoundRobinEnvironmentStd<Comm, C>
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





impl<Comm, C> Environment for RoundRobinEnvironmentStd<Comm, C>
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

impl<Comm, C> CommunicatingEnvironment<ServerDealMessage, ClientDealMessage, BridgeErrorStd> for RoundRobinEnvironmentStd<Comm, C>
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