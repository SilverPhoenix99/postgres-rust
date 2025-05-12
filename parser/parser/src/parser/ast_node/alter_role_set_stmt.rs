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

    pub fn role(&self) -> &OneOrAll<RoleSpec> {
        &self.role
    }

    pub fn database(&self) -> &Option<Str> {
        &self.database
    }

    pub fn set_stmt(&self) -> &SetResetClause {
        &self.set_stmt
    }
}

use crate::parser::ast_node::OneOrAll;
use crate::parser::ast_node::RoleSpec;
use crate::parser::ast_node::SetResetClause;
use postgres_basics::Str;
