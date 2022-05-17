use arrayvec::ArrayVec;
use crate::player::side::Side;
use crate::play::trick::Trick;
use crate::play::deck::QUARTER_SIZE;
use crate::card::figure::Figure;
use crate::card::suit::Suit;
use crate::player::role::PlayRole;

#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct Player<F: Figure, S: Suit>{
    id: u8,
    name: String,
    play_role: Option<PlayRole>,
    tricks_taken: ArrayVec<Trick<F, S>, QUARTER_SIZE>,
    side: Side


}
impl<F: Figure, S: Suit> Player<F, S>{
    pub fn new(id: u8, name: String, side: Side) -> Self{
        Self{id, name, play_role: None, tricks_taken: ArrayVec::new(), side}
    }
    pub fn id(&self) -> u8{
        self.id
    }
    pub fn name(&self) -> &str{
        &self.name
    }
    pub fn play_role(&self) -> &Option<PlayRole>{
        &self.play_role
    }
    pub fn tricks_taken(&self) -> &ArrayVec<Trick<F, S>, QUARTER_SIZE>{
        &self.tricks_taken
    }
}