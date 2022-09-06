
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus{
    Ready,
    NotReady,
    Absent
}

impl Default for PlayerStatus{
    fn default() -> Self {
        Self::NotReady
    }
}