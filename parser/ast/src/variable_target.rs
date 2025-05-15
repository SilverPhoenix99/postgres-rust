#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VariableTarget {
    All,
    SessionAuthorization,
    TransactionIsolation,
    TimeZone,
    Variable {
        // Name, possibly qualified, separated by dots
        name: QualifiedName
    },
}

use postgres_basics::QualifiedName;
