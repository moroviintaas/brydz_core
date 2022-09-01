use karty::cards::CardStd;
use karty::suits::SuitStd;
use crate::bidding::Call;

pub enum Action{
    MakeCall(Call<SuitStd>),
    PlayCard(CardStd)
}