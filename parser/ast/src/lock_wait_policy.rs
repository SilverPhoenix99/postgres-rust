#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum LockWaitPolicy {
    #[default]
    Block,
    WaitSkip,
    WaitError,
}
