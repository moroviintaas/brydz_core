use std::error::Error;

mod std_channel;
pub use std_channel::SyncComm;
#[cfg(feature = "async")]
mod tokio_channel;
#[cfg(feature = "async")]
pub use tokio_channel::*;
use crate::error::BridgeErrorStd;
use crate::protocol::{ClientDealMessage, ServerDealMessage};

pub trait CommunicationEnd< OT, IT,E: Error>{

    //type Mirror: CommunicationEnd<IT, OT, E>;

    fn send(&self, message: OT) -> Result<(), E>;
    fn recv(&mut self) -> Result<IT, E>;
    fn try_recv(&mut self) -> Result<IT, E>;

    //fn new_pair() ->  (Self, Self::Mirror);


}

pub trait CommunicationEndStd: CommunicationEnd<ServerDealMessage, ClientDealMessage, BridgeErrorStd>{}