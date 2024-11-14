#[derive(Debug, Clone, PartialEq)]
pub enum RawStmt {
    AlterDefaultPrivilegesStmt(Box<AlterDefaultPrivilegesStmt>),
    AlterEventTrigStmt(AlterEventTrigStmt),
    AlterObjectSchemaStmt(AlterObjectSchemaStmt),
    AlterOwnerStmt(AlterOwnerStmt),
    AlterRoleStmt(AlterRoleStmt),
    CheckPoint,
    ClosePortalStmt(OneOrAll),
    DeallocateStmt(OneOrAll),
    DiscardStmt(DiscardStmt),
    ListenStmt(Str),
    LoadStmt(Box<str>),
    NotifyStmt(NotifyStmt),
    PrepareStmt(Box<PrepareStmt>),
    PrepareTransactionStmt(Box<str>),
    ReassignOwnedStmt(ReassignOwnedStmt),
    RefreshCollationVersionStmt(QualifiedName),
    RenameStmt(RenameStmt),
    TransactionStmt(TransactionStmt),
    UnlistenStmt(OneOrAll),
    VariableShowStmt(VariableShowStmt),
}

impl_from!(box AlterDefaultPrivilegesStmt for RawStmt);
impl_from!(AlterEventTrigStmt for RawStmt);
impl_from!(AlterObjectSchemaStmt for RawStmt);
impl_from!(AlterOwnerStmt for RawStmt);
impl_from!(AlterRoleStmt for RawStmt);
impl_from!(DiscardStmt for RawStmt);
impl_from!(NotifyStmt for RawStmt);
impl_from!(box PrepareStmt for RawStmt);
impl_from!(ReassignOwnedStmt for RawStmt);
impl_from!(RenameStmt for RawStmt);
impl_from!(TransactionStmt for RawStmt);
impl_from!(VariableShowStmt for RawStmt);

use crate::parser::ast_node::{
    impl_from,
    AlterDefaultPrivilegesStmt,
    AlterEventTrigStmt,
    AlterObjectSchemaStmt,
    AlterOwnerStmt,
    AlterRoleStmt,
    DiscardStmt,
    NotifyStmt,
    OneOrAll,
    PrepareStmt,
    QualifiedName,
    ReassignOwnedStmt,
    RenameStmt,
    TransactionStmt,
    VariableShowStmt
};
use postgres_basics::Str;
