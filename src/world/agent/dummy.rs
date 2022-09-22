use crate::error::BridgeErrorStd;
use crate::player::situation::Situation;
use crate::protocol::{ClientDealMessage, DealNotify, ServerDealMessage};
use crate::protocol::ClientControlMessage::{IamReady, Quit};
use crate::protocol::ClientDealInformation::ShowHand;
use crate::world::agent::AutomaticAgent;
use crate::world::comm::CommunicationEnd;

pub struct DummyBot<Comm: CommunicationEnd<ClientDealMessage, ServerDealMessage, BridgeErrorStd>>{
    situation: Situation,
    comm: Comm,
}

impl<Comm> DummyBot<Comm>
where Comm: CommunicationEnd<ClientDealMessage, ServerDealMessage, BridgeErrorStd>{
    pub fn new(comm: Comm, situation: Situation) -> Self{
        Self{comm, situation}
    }
}

impl<Comm> AutomaticAgent<BridgeErrorStd> for DummyBot<Comm>
where Comm: CommunicationEnd<ClientDealMessage, ServerDealMessage, BridgeErrorStd>{
    fn run(&mut self) -> Result<(), BridgeErrorStd> {
        self.comm.send(IamReady.into())?;
        loop{
            match self.comm.recv()?{
                ServerDealMessage::Notify(notify) => match notify{
                    DealNotify::ShowYourHand => {
                        self.comm.send(ShowHand(self.situation.hand().clone()).into())?
                    },
                    DealNotify::DealClosed => {
                        self.comm.send(Quit.into()).unwrap_or(());
                        return Ok(())
                    },
                    _ => {}
                }
                ServerDealMessage::Info(_) => {}
                ServerDealMessage::Control(_) => {}
            }
        }
    }
}