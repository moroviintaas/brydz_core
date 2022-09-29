#[cfg(test)]
mod tests{
    use karty::cards::STANDARD_DECK;
    use crate::distribution::hand::BridgeHand;
    use crate::protocol::{DealNotify, MAX_PROTOCOL_MESSAGE_SIZE, ServerDealMessage, ServerMessage};
    #[cfg(feature = "speedy")]
    use speedy::Writable;

    #[test]
    #[cfg(feature = "speedy")]
    fn test_size_of_protocol_messages(){
        let mut deck = Vec::from(STANDARD_DECK);
        let hand_1 = BridgeHand::drain_full_from_vec(&mut deck).unwrap();
        let m1 = ServerMessage::Deal(ServerDealMessage::Notify(DealNotify::DummyPlacedHand(hand_1)));
        let m1_serialized = m1.write_to_vec().unwrap();
        assert_eq!(m1_serialized.len(), MAX_PROTOCOL_MESSAGE_SIZE);
    }
}