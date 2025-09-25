#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum RawStmt {
    AlterDatabaseRefreshCollStmt(Str),
    #[from]
    AlterDatabaseSetStmt(AlterDatabaseSetStmt),
    #[from]
    AlterDatabaseStmt(AlterDatabaseStmt),
    #[from(AlterDefaultPrivilegesStmt)]
    AlterDefaultPrivilegesStmt(Box<AlterDefaultPrivilegesStmt>),
    #[from]
    AlterEventTrigStmt(AlterEventTrigStmt),
    #[from]
    AlterExtensionContentsStmt(AlterExtensionContentsStmt),
    #[from]
    AlterExtensionStmt(AlterExtensionStmt),
    #[from]
    AlterFunctionStmt(AlterFunctionStmt),
    #[from]
    AlterObjectDependsStmt(AlterObjectDependsStmt),
    #[from]
    AlterObjectSchemaStmt(AlterObjectSchemaStmt),
    #[from]
    AlterOwnerStmt(AlterOwnerStmt),
    #[from]
    AlterRoleSetStmt(AlterRoleSetStmt),
    #[from]
    AlterRoleStmt(AlterRoleStmt),
    #[from]
    AlterSystemStmt(AlterSystemStmt),
    #[from]
    AlterUserMappingStmt(AlterUserMappingStmt),
    CheckPointStmt(Option<Vec<UtilityOption>>),
    ClosePortalStmt(OneOrAll<Str>),
    #[from]
    CommentStmt(CommentStmt),
    #[from]
    ConstraintsSetStmt(ConstraintsSetStmt),
    #[from]
    CreateAccessMethodStmt(CreateAccessMethodStmt),
    #[from]
    CreateCastStmt(CreateCastStmt),
    #[from]
    CreateConversionStmt(CreateConversionStmt),
    #[from]
    CreateDatabaseStmt(CreateDatabaseStmt),
    #[from]
    CreateRoleStmt(CreateRoleStmt),
    #[from]
    CreateUserMappingStmt(CreateUserMappingStmt),
    DeallocateStmt(OneOrAll<Str>),
    #[from]
    DiscardStmt(DiscardStmt),
    ListenStmt(Str),
    LoadStmt(Box<str>),
    #[from]
    NotifyStmt(NotifyStmt),
    #[from(PrepareStmt)]
    PrepareStmt(Box<PrepareStmt>),
    PrepareTransactionStmt(Box<str>),
    #[from]
    ReassignOwnedStmt(ReassignOwnedStmt),
    RefreshCollationVersionStmt(QualifiedName),
    #[from]
    RenameStmt(RenameStmt),
    #[from]
    SecurityLabelStmt(SecurityLabelStmt),
    #[from]
    TransactionStmt(TransactionStmt),
    UnlistenStmt(OneOrAll<Str>),
    VariableResetStmt(VariableTarget),
    #[from]
    VariableSetStmt(VariableSetStmt),
    VariableShowStmt(VariableTarget),
}

impl From<RoleStmt> for RawStmt {
    fn from(value: RoleStmt) -> Self {
        match value {
            RoleStmt::Rename { role_name, new_name } => {
                RenameStmt::new(
                    RenameTarget::Role(role_name),
                    new_name
                ).into()
            }
            RoleStmt::AlterOptions(stmt) => stmt.into(),
            RoleStmt::AlterConfig(stmt) => stmt.into(),
            RoleStmt::AlterUserMappings(stmt) => stmt.into(),
            RoleStmt::Create(stmt) => stmt.into(),
            RoleStmt::CreateUserMapping(stmt) => stmt.into(),
        }
    }
}

impl From<DatabaseStmt> for RawStmt {
    fn from(value: DatabaseStmt) -> Self {
        match value {
            DatabaseStmt::RefreshCollation(name) => Self::AlterDatabaseRefreshCollStmt(name),
            DatabaseStmt::AlterOwner { db_name, new_owner } => {
                AlterOwnerStmt::new(
                    AlterOwnerTarget::Database(db_name),
                    new_owner
                ).into()
            },
            DatabaseStmt::Rename { db_name, new_name } => {
                RenameStmt::new(
                    RenameTarget::Database(db_name),
                    new_name
                ).into()
            },
            DatabaseStmt::AlterDatabase(stmt) => stmt.into(),
            DatabaseStmt::AlterDatabaseSet(stmt) => stmt.into(),
        }
    }
}

use crate::AlterEventTrigStmt;
use crate::AlterExtensionContentsStmt;
use crate::AlterExtensionStmt;
use crate::AlterFunctionStmt;
use crate::AlterObjectDependsStmt;
use crate::AlterObjectSchemaStmt;
use crate::AlterOwnerStmt;
use crate::AlterSystemStmt;
use crate::CommentStmt;
use crate::ConstraintsSetStmt;
use crate::CreateAccessMethodStmt;
use crate::CreateCastStmt;
use crate::CreateConversionStmt;
use crate::DiscardStmt;
use crate::NotifyStmt;
use crate::PrepareStmt;
use crate::ReassignOwnedStmt;
use crate::RenameStmt;
use crate::RenameTarget;
use crate::SecurityLabelStmt;
use crate::UtilityOption;
use crate::VariableSetStmt;
use crate::{AlterDefaultPrivilegesStmt, AlterOwnerTarget};
use derive_more::From;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_database_stmt_ast::AlterDatabaseStmt;
use pg_database_stmt_ast::CreateDatabaseStmt;
use pg_database_stmt_ast::{AlterDatabaseSetStmt, DatabaseStmt};
use pg_generic_set_ast::VariableTarget;
use pg_role_ast::AlterRoleSetStmt;
use pg_role_ast::AlterRoleStmt;
use pg_role_ast::AlterUserMappingStmt;
use pg_role_ast::CreateRoleStmt;
use pg_role_ast::CreateUserMappingStmt;
use pg_role_ast::RoleStmt;
use pg_sink_ast::OneOrAll;
use pg_transaction_stmt_ast::TransactionStmt;
