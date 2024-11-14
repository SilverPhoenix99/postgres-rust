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
    CommitPrepared(Box<str>),
    Savepoint(Str),
    Release(Str),
    Prepare(Box<str>),
    Rollback { chain: bool },
    RollbackTo(Str),
    RollbackPrepared(Box<str>),
}

use postgres_basics::Str;
