 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockClauseStrength {
    ForKeyShare    = 1,
    ForShare       = 2,
    ForNoKeyUpdate = 3,
    ForUpdate      = 4,
}
