use thiserror;
use karty::symbol::CardSymbol;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub enum FuzzyCardSetErrorGen<Crd: CardSymbol>{
    #[error("Requested card is not in hand")]
    CardNotInHand(Crd),
    #[error("Parsing FuzzyCardSet")]
    Parse,
    #[error("Bad probabilities sum, expected: {0}, found {1}")]
    BadProbabilitiesSum(f32, f32),
    #[error("Downscale factor {0} is forbidden")]
    ForbiddenDownscale(f32),
    #[error("Probability is bad: {0} (over 1.0)")]
    ProbabilityOverOne(f32),
    #[error("Probability is bad: {0} (below 0.0)")]
    ProbabilityBelowZero(f32),
    #[error("Probability is bad: {0} (unspecified)")]
    BadProbability(f32),

}