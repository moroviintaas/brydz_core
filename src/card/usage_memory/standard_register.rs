use crate::card::Card;
use crate::card::figure::FigureStd;
use crate::card::register::CardRegister;
use crate::card::suit::SuitStd;

#[derive(Debug)]
pub struct CardUsageRegStd{
    memory: u64
}
impl Default for CardUsageRegStd{
    fn default() -> Self {
        Self{memory: 0}
    }
}
impl CardRegister<FigureStd, SuitStd> for CardUsageRegStd{

    fn mark_used(&mut self, card: &Card<FigureStd, SuitStd>) {
        self.memory = self.memory | card.mask();
    }

    fn is_card_used(&self, card: &Card<FigureStd, SuitStd>) -> bool {
        !matches!(self.memory & card.mask(), 0)
    }
}