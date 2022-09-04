
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus{
    Ready,
    NotReady,
    Quit
}

impl Default for PlayerStatus{
    fn default() -> Self {
        Self::NotReady
    }
}