#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VariableShowStmt {
    All,
    SessionAuthorization,
    TransactionIsolation,
    TimeZone,
    // Name, possibly qualified, separated by dots
    Name(QualifiedName),
}

use crate::parser::ast_node::QualifiedName;
