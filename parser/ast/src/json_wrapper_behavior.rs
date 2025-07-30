#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JsonWrapperBehavior {
    Without,
    Conditional,
    Unconditional,
}
