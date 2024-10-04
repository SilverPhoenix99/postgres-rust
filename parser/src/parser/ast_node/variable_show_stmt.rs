#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VariableShowStmt {
    All,
    SessionAuthorization,
    TransactionIsolation,
    TimeZone,
    // Name, possibly qualified, separated by dots
    Name(Vec<CowStr>),
}

use super::CowStr;
