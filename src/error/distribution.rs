#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistributionError{
    TooFewCards(usize)
}