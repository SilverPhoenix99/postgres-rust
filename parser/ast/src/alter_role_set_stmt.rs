#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterRoleSetStmt {
    role: OneOrAll<RoleSpec>,
    database: Option<Str>,
    set_stmt: SetResetClause
}

impl AlterRoleSetStmt {
    pub fn new(role: OneOrAll<RoleSpec>, database: Option<Str>, set_stmt: SetResetClause) -> Self {
        Self { role, database, set_stmt }
    }

    pub fn role(&self) -> OneOrAll<&RoleSpec> {
        self.role.as_ref()
    }

    pub fn database(&self) -> Option<&str> {
        self.database.as_deref()
    }

    pub fn set_stmt(&self) -> &SetResetClause {
        &self.set_stmt
    }
}

use crate::OneOrAll;
use crate::SetResetClause;
use pg_basics::Str;
use pg_sink_ast::RoleSpec;
