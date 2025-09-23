#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum RoleStmt {

    // ALTER
    Rename {
        role_name: Str,
        new_name: Str,
    },
    AlterOptions(AlterRoleStmt),
    AlterConfig(AlterRoleSetStmt),
    AlterUserMappings(AlterUserMappingStmt),

    // CREATE
    Create(CreateRoleStmt),
    CreateUserMapping(CreateUserMappingStmt),
}

use crate::AlterRoleSetStmt;
use crate::AlterRoleStmt;
use crate::AlterUserMappingStmt;
use crate::CreateRoleStmt;
use crate::CreateUserMappingStmt;
use derive_more::From;
use pg_basics::Str;
