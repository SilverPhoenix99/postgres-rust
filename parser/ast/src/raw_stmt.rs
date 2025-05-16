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
    CreateAccessMethodStmt(CreateAccessMethodStmt),
    CreateCastStmt(CreateCastStmt),
    CreateConversionStmt(CreateConversionStmt),
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
impl_from!(CreateAccessMethodStmt for RawStmt);
impl_from!(CreateCastStmt for RawStmt);
impl_from!(CreateConversionStmt for RawStmt);
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

use crate::AlterDatabaseSetStmt;
use crate::AlterDatabaseStmt;
use crate::AlterDefaultPrivilegesStmt;
use crate::AlterEventTrigStmt;
use crate::AlterExtensionContentsStmt;
use crate::AlterExtensionStmt;
use crate::AlterFunctionStmt;
use crate::AlterObjectDependsStmt;
use crate::AlterObjectSchemaStmt;
use crate::AlterOwnerStmt;
use crate::AlterRoleSetStmt;
use crate::AlterRoleStmt;
use crate::AlterSystemStmt;
use crate::AlterUserMappingStmt;
use crate::CommentStmt;
use crate::ConstraintsSetStmt;
use crate::CreateAccessMethodStmt;
use crate::CreateCastStmt;
use crate::CreateConversionStmt;
use crate::CreateDatabaseStmt;
use crate::CreateRoleStmt;
use crate::CreateUserMappingStmt;
use crate::DiscardStmt;
use crate::NotifyStmt;
use crate::OneOrAll;
use crate::PrepareStmt;
use crate::ReassignOwnedStmt;
use crate::RenameStmt;
use crate::SecurityLabelStmt;
use crate::TransactionStmt;
use crate::VariableSetStmt;
use crate::VariableTarget;
use pg_basics::impl_from;
use pg_basics::QualifiedName;
use pg_basics::Str;
