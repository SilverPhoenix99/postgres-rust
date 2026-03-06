#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterDatabaseSetStmt {
    name: Str,
    option: SetResetClause
}

impl AlterDatabaseSetStmt {
    pub fn new<T: Into<Str>>(name: T, option: SetResetClause) -> Self {
        Self {
            name: name.into(),
            option
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn option(&self) -> &SetResetClause {
        &self.option
    }
}

use pg_basics::Str;
use pg_generic_set_ast::SetResetClause;
