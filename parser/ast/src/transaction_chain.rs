#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TransactionChain {
    #[default]
    NoChain,
    WithChain,
}
