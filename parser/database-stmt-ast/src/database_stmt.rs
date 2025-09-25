#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum DatabaseStmt {
    RefreshCollation(Str),
    AlterOwner {
        db_name: Str,
        new_owner: RoleSpec,
    },
    Rename {
        db_name: Str,
        new_name: Str,
    },
    #[from]
    AlterDatabase(AlterDatabaseStmt),
    #[from]
    AlterDatabaseSet(AlterDatabaseSetStmt),
}

use crate::AlterDatabaseSetStmt;
use crate::AlterDatabaseStmt;
use derive_more::From;
use pg_basics::Str;
use pg_sink_ast::RoleSpec;
