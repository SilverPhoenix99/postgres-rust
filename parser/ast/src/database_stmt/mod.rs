pub(super) mod alterdb_option;
pub(super) mod alterdb_option_kind;
pub(super) mod createdb_option;
pub(super) mod createdb_option_kind;
pub(super) mod createdb_option_value;
pub(super) mod database_stmt_option;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DatabaseStmt {
    db_name: Str,
    stmt: DatabaseStmtOption,
}

impl DatabaseStmt {
    pub fn new<T: Into<Str>>(db_name: T, stmt: DatabaseStmtOption) -> Self {
        Self {
            db_name: db_name.into(),
            stmt
        }
    }

    pub fn db_name(&self) -> &str {
        &self.db_name
    }

    pub fn stmt(&self) -> &DatabaseStmtOption {
        &self.stmt
    }
}

use crate::DatabaseStmtOption;
use pg_basics::Str;
