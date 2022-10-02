use std::error::Error;
use std::marker::PhantomData;

use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::{SendError, TryRecvError};
use crate::error::CommError;
use crate::error::CommError::RecvError;
use crate::world::comm::CommunicationEnd;

#[derive(Debug)]
/// # Example:
/// ```
/// use std::thread::spawn;
/// use brydz_core::error::BridgeErrorStd;
/// use brydz_core::world::comm::CommunicationEnd;
/// use brydz_core::world::comm::TokioComm;
/// let (mut com1, mut com2) = TokioComm::<String, String, BridgeErrorStd>::new_pair();
/// let h1 = spawn(move || {
///     com1.send(format!("Hello")).unwrap();
/// });
/// let r = com2.recv().unwrap();
/// assert_eq!(r, format!("Hello"));
/// ```
pub struct TokioComm<OT, IT, E: Error>{
    sender: UnboundedSender<OT>,
    receiver: UnboundedReceiver<IT>,
    rt: Runtime,
    phantom: PhantomData<E>
}

impl<OT, IT, E: Error> TokioComm<OT, IT, E>
where Self: CommunicationEnd<OT, IT, E>{
    pub fn new_pair() -> (Self, TokioComm<IT, OT, E>){
        let (tx_1, rx_1) = unbounded_channel();
        let (tx_2, rx_2) = unbounded_channel();
        let rt1 = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        let rt2 = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        (Self{sender: tx_1, receiver: rx_2, phantom: PhantomData::default(), rt: rt1},
        TokioComm{sender: tx_2, receiver: rx_1, phantom: PhantomData::default(), rt: rt2})
    }
}


impl<OT, IT, E> CommunicationEnd<OT, IT, E> for TokioComm<OT, IT, E>
where E: Error  + From<SendError<OT>> + From<TryRecvError> + From<SendError<IT>> + From<CommError>{
//where E: Error    {
    fn send(&mut self, message: OT) -> Result<(), E> {
        self.sender.send(message).map_err(|e| e.into())
    }

    fn recv(&mut self) -> Result<IT, E> {
        self.rt.block_on(self.receiver.recv()).ok_or_else(|| RecvError.into())
    }

    fn try_recv(&mut self) -> Result<IT, E> {
        self.receiver.try_recv().map_err(|e| e.into())
    }
}


#[cfg(test)]
mod test{
    use std::thread::spawn;
    use crate::error::BridgeErrorStd;
    use crate::world::comm::{CommunicationEnd, TokioComm};

    #[test]
    fn t1(){
        let (mut com1, mut com2) = TokioComm::<String, String, BridgeErrorStd>::new_pair();
        let _h1 = spawn(move || {
            com1.send(format!("Hello")).unwrap();
        });
        let r = com2.recv().unwrap();
        assert_eq!(r, format!("Hello"));

    }
}