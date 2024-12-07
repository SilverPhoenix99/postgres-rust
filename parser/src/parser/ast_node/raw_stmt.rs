#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RawStmt {
    AlterDatabaseStmt(AlterDatabaseStmt),
    AlterDatabaseRefreshCollStmt(Str),
    AlterDefaultPrivilegesStmt(Box<AlterDefaultPrivilegesStmt>),
    AlterEventTrigStmt(AlterEventTrigStmt),
    AlterObjectSchemaStmt(AlterObjectSchemaStmt),
    AlterOwnerStmt(AlterOwnerStmt),
    AlterRoleStmt(AlterRoleStmt),
    CheckPoint,
    ClosePortalStmt(OneOrAll),
    CreateDatabaseStmt(CreateDatabaseStmt),
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
impl_from!(AlterDatabaseStmt for RawStmt);
impl_from!(AlterEventTrigStmt for RawStmt);
impl_from!(AlterObjectSchemaStmt for RawStmt);
impl_from!(AlterOwnerStmt for RawStmt);
impl_from!(AlterRoleStmt for RawStmt);
impl_from!(CreateDatabaseStmt for RawStmt);
impl_from!(DiscardStmt for RawStmt);
impl_from!(NotifyStmt for RawStmt);
impl_from!(box PrepareStmt for RawStmt);
impl_from!(ReassignOwnedStmt for RawStmt);
impl_from!(RenameStmt for RawStmt);
impl_from!(TransactionStmt for RawStmt);
impl_from!(VariableShowStmt for RawStmt);

use crate::parser::ast_node::AlterDatabaseStmt;
use crate::parser::ast_node::AlterDefaultPrivilegesStmt;
use crate::parser::ast_node::AlterEventTrigStmt;
use crate::parser::ast_node::AlterObjectSchemaStmt;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterRoleStmt;
use crate::parser::ast_node::CreateDatabaseStmt;
use crate::parser::ast_node::DiscardStmt;
use crate::parser::ast_node::NotifyStmt;
use crate::parser::ast_node::OneOrAll;
use crate::parser::ast_node::PrepareStmt;
use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::ReassignOwnedStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::ast_node::VariableShowStmt;
use postgres_basics::impl_from;
use postgres_basics::Str;
