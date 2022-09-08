use std::sync::mpsc::{Receiver, Sender};
use crate::error::BridgeErrorStd;
use crate::player::situation::Situation;
use crate::protocol::{ClientDealMessage, DealNotify, ServerDealMessage};
use crate::protocol::ClientControlMessage::{IamReady, Quit};
use crate::protocol::ClientDealInformation::ShowHand;
use crate::world::agent::AutomaticAgent;

pub struct ChannelDummy{
    sender: Sender<ClientDealMessage>,
    receiver: Receiver<ServerDealMessage>,
    situation: Situation,
}

impl ChannelDummy{
    pub fn new(sender: Sender<ClientDealMessage>, receiver: Receiver<ServerDealMessage>, situation: Situation) -> Self{
        Self{sender, receiver, situation}
    }
}

impl AutomaticAgent<BridgeErrorStd> for ChannelDummy{
    fn run(&mut self) -> Result<(), BridgeErrorStd> {
        self.sender.send(IamReady.into())?;
        loop{
            match self.receiver.recv()?{
                ServerDealMessage::Notify(notify) => match notify{
                    DealNotify::YourMove => {
                        self.sender.send(ShowHand(self.situation.hand().clone()).into())?
                    },
                    DealNotify::DealClosed => {
                        self.sender.send(Quit.into()).unwrap_or(());
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