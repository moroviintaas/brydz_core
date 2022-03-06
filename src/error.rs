use std::fmt::{Display, Error, Formatter};
use crate::card::ReleasedCard;
use crate::player::Player;
use crate::player::role::PlayRole;

#[derive(Debug, Clone)]
pub enum BridgeError{
    CardSlotAlreadyUsed(ReleasedCard),
    CardAlreadyUsed(ReleasedCard),
    PlayerAlreadyPlayed,
    ViolatedPlayOrder(Player, Player),
    PlayerWithoutPlayRole,
    MissingCard(PlayRole)
}

impl Display for BridgeError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}