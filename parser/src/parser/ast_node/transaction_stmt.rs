use crate::parser::ast_node::CowStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TransactionMode {
    IsolationLevel(IsolationLevel),
    ReadOnly,
    ReadWrite,
    Deferrable,
    NotDeferrable,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TransactionStmt {
    Begin(Vec<TransactionMode>),
    /// Semantically identical to `BEGIN`.
    Start(Vec<TransactionMode>),
    Commit { chain: bool },
    CommitPrepared(String),
    Savepoint(CowStr),
    Release(CowStr),
    Prepare(String),
    Rollback { chain: bool },
    RollbackTo(CowStr),
    RollbackPrepared(String),
}