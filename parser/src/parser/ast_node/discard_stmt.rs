#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DiscardStmt {
    All,
    Plans,
    Sequences,
    Temporary,
}