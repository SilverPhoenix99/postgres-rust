#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RawStmt {
    AlterDatabaseRefreshCollStmt(Str),
    AlterDatabaseSetStmt(AlterDatabaseSetStmt),
    AlterDatabaseStmt(AlterDatabaseStmt),
    AlterDefaultPrivilegesStmt(Box<AlterDefaultPrivilegesStmt>),
    AlterEventTrigStmt(AlterEventTrigStmt),
    AlterExtensionContentsStmt(AlterExtensionContentsStmt),
    AlterExtensionStmt(AlterExtensionStmt),
    AlterFunctionStmt(AlterFunctionStmt),
    AlterObjectDependsStmt(AlterObjectDependsStmt),
    AlterObjectSchemaStmt(AlterObjectSchemaStmt),
    AlterOwnerStmt(AlterOwnerStmt),
    AlterRoleSetStmt(AlterRoleSetStmt),
    AlterRoleStmt(AlterRoleStmt),
    AlterSystemStmt(AlterSystemStmt),
    AlterUserMappingStmt(AlterUserMappingStmt),
    CheckPoint,
    ClosePortalStmt(OneOrAll<Str>),
    CommentStmt(CommentStmt),
    ConstraintsSetStmt(ConstraintsSetStmt),
    CreateDatabaseStmt(CreateDatabaseStmt),
    CreateRoleStmt(CreateRoleStmt),
    CreateUserMappingStmt(CreateUserMappingStmt),
    DeallocateStmt(OneOrAll<Str>),
    DiscardStmt(DiscardStmt),
    ListenStmt(Str),
    LoadStmt(Box<str>),
    NotifyStmt(NotifyStmt),
    PrepareStmt(Box<PrepareStmt>),
    PrepareTransactionStmt(Box<str>),
    ReassignOwnedStmt(ReassignOwnedStmt),
    RefreshCollationVersionStmt(QualifiedName),
    RenameStmt(RenameStmt),
    SecurityLabelStmt(SecurityLabelStmt),
    TransactionStmt(TransactionStmt),
    UnlistenStmt(OneOrAll<Str>),
    VariableResetStmt(VariableTarget),
    VariableSetStmt(VariableSetStmt),
    VariableShowStmt(VariableTarget),
}

impl_from!(AlterDatabaseSetStmt for RawStmt);
impl_from!(AlterDatabaseStmt for RawStmt);
impl_from!(box AlterDefaultPrivilegesStmt for RawStmt);
impl_from!(AlterEventTrigStmt for RawStmt);
impl_from!(AlterExtensionContentsStmt for RawStmt);
impl_from!(AlterExtensionStmt for RawStmt);
impl_from!(AlterFunctionStmt for RawStmt);
impl_from!(AlterObjectDependsStmt for RawStmt);
impl_from!(AlterObjectSchemaStmt for RawStmt);
impl_from!(AlterOwnerStmt for RawStmt);
impl_from!(AlterRoleSetStmt for RawStmt);
impl_from!(AlterRoleStmt for RawStmt);
impl_from!(AlterSystemStmt for RawStmt);
impl_from!(AlterUserMappingStmt for RawStmt);
impl_from!(CommentStmt for RawStmt);
impl_from!(ConstraintsSetStmt for RawStmt);
impl_from!(CreateDatabaseStmt for RawStmt);
impl_from!(CreateRoleStmt for RawStmt);
impl_from!(CreateUserMappingStmt for RawStmt);
impl_from!(DiscardStmt for RawStmt);
impl_from!(NotifyStmt for RawStmt);
impl_from!(box PrepareStmt for RawStmt);
impl_from!(ReassignOwnedStmt for RawStmt);
impl_from!(RenameStmt for RawStmt);
impl_from!(SecurityLabelStmt for RawStmt);
impl_from!(TransactionStmt for RawStmt);
impl_from!(VariableSetStmt for RawStmt);

use crate::parser::ast_node::AlterDatabaseSetStmt;
use crate::parser::ast_node::AlterDatabaseStmt;
use crate::parser::ast_node::AlterDefaultPrivilegesStmt;
use crate::parser::ast_node::AlterEventTrigStmt;
use crate::parser::ast_node::AlterExtensionContentsStmt;
use crate::parser::ast_node::AlterExtensionStmt;
use crate::parser::ast_node::AlterFunctionStmt;
use crate::parser::ast_node::AlterObjectDependsStmt;
use crate::parser::ast_node::AlterObjectSchemaStmt;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterRoleSetStmt;
use crate::parser::ast_node::AlterRoleStmt;
use crate::parser::ast_node::AlterSystemStmt;
use crate::parser::ast_node::AlterUserMappingStmt;
use crate::parser::ast_node::CommentStmt;
use crate::parser::ast_node::ConstraintsSetStmt;
use crate::parser::ast_node::CreateDatabaseStmt;
use crate::parser::ast_node::CreateRoleStmt;
use crate::parser::ast_node::CreateUserMappingStmt;
use crate::parser::ast_node::DiscardStmt;
use crate::parser::ast_node::NotifyStmt;
use crate::parser::ast_node::OneOrAll;
use crate::parser::ast_node::PrepareStmt;
use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::ReassignOwnedStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::SecurityLabelStmt;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::ast_node::VariableSetStmt;
use crate::parser::ast_node::VariableTarget;
use postgres_basics::impl_from;
use postgres_basics::Str;
