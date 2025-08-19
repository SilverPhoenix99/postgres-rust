#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterRoleSetStmt {
    role: OneOrAll<RoleSpec>,
    database: Option<Str>,
    set_stmt: SetResetClause,
}

impl AlterRoleSetStmt {
    pub fn new(role: OneOrAll<RoleSpec>, set_stmt: SetResetClause) -> Self {
        Self {
            role,
            database: None,
            set_stmt
        }
    }

    pub fn role(&self) -> OneOrAll<&RoleSpec> {
        self.role.as_ref()
    }

    pub fn set_database(&mut self, database: Option<Str>) -> &mut Self {
        self.database = database;
        self
    }

    pub fn with_database<T: Into<Str>>(mut self, database: T) -> Self {
        self.database = Some(database.into());
        self
    }

    pub fn database(&self) -> Option<&str> {
        self.database.as_deref()
    }

    pub fn set_stmt(&self) -> &SetResetClause {
        &self.set_stmt
    }
}

use pg_basics::Str;
use pg_generic_set_ast::SetResetClause;
use pg_sink_ast::OneOrAll;
use pg_sink_ast::RoleSpec;
