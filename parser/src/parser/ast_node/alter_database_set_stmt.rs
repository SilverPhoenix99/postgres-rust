#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterDatabaseSetStmt {
    name: Str,
    option: SetResetClause
}

impl AlterDatabaseSetStmt {
    pub fn new<T>(name: T, option: SetResetClause) -> Self
    where
        Str: From<T>
    {
        Self {
            name: name.into(),
            option
        }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn option(&self) -> &SetResetClause {
        &self.option
    }
}

use crate::parser::ast_node::SetResetClause;
use postgres_basics::Str;
