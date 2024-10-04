mod alter_event_trig_stmt;
mod alter_role_stmt;
mod ast_literal;
mod discard_stmt;
mod notify_stmt;
mod numeric_spec;
mod reassign_owned_stmt;
mod role_spec;
mod system_type;
mod transaction_stmt;
mod variable_show_stmt;

pub(super) use self::system_type::CharacterSystemType;
pub use self::{
    alter_event_trig_stmt::{AlterEventTrigStmt, EventTriggerState},
    alter_role_stmt::{AlterRoleAction, AlterRoleOption, AlterRoleStmt},
    ast_literal::AstLiteral,
    discard_stmt::DiscardStmt,
    notify_stmt::NotifyStmt,
    numeric_spec::NumericSpec,
    reassign_owned_stmt::ReassignOwnedStmt,
    role_spec::RoleSpec,
    system_type::SystemType,
    transaction_stmt::{IsolationLevel, TransactionMode, TransactionStmt},
    variable_show_stmt::VariableShowStmt,
};

type CowStr = Cow<'static, str>;

/// Generates `From` impls, where the input is wrapped in an output enum variant.
macro_rules! impl_from {
    ($variant:ident for $for_:ident) => {
        impl_from!($variant for $for_ => $variant);
    };
    ($from:ident for $for_:ident => $variant:ident) => {
        impl From<$from> for $for_ {
            #[inline(always)]
            fn from(value: $from) -> Self {
                Self::$variant(value)
            }
        }
    };
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OneOrAll {
    All,
    Name(CowStr),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RenameTarget {
    Aggregate(/* TODO */),
    Collation(/* TODO */),
    Conversion(/* TODO */),
    Database(/* TODO */),
    Domain(/* TODO */),
    DomainConstraint(/* TODO */),
    EventTrigger(CowStr),
    ForeignDataWrapper(/* TODO */),
    ForeignServer(/* TODO */),
    ForeignTable(/* TODO */),
    ForeignTableColumn(/* TODO */),
    Function(/* TODO */),
    Index(/* TODO */),
    Language(CowStr),
    MaterializedView(/* TODO */),
    MaterializedViewColumn(/* TODO */),
    OperatorClass(/* TODO */),
    OperatorFamily(/* TODO */),
    Policy(/* TODO */),
    Procedure(/* TODO */),
    Publication(/* TODO */),
    Role(CowStr),
    Routine(/* TODO */),
    Rule(/* TODO */),
    Schema(/* TODO */),
    Sequence(/* TODO */),
    StatisticExt(/* TODO */),
    Subscription(/* TODO */),
    Table(/* TODO */),
    TableColumn(/* TODO */),
    TableConstraint(/* TODO */),
    Tablespace(/* TODO */),
    TextSearchConfiguration(/* TODO */),
    TextSearchDictionary(/* TODO */),
    TextSearchParser(/* TODO */),
    TextSearchTemplate(/* TODO */),
    Trigger(/* TODO */),
    Type(/* TODO */),
    TypeAttribute(/* TODO */),
    View(/* TODO */),
    ViewColumn(/* TODO */),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RenameStmt {
    target: RenameTarget,
    new_name: CowStr,
}

impl RenameStmt {
    #[inline(always)]
    pub fn new(target: RenameTarget, new_name: CowStr) -> Self {
        Self { target, new_name }
    }

    #[inline(always)]
    pub fn target(&self) -> &RenameTarget {
        &self.target
    }

    #[inline(always)]
    pub fn new_name(&self) -> &CowStr {
        &self.new_name
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterOwnerTarget {
    Aggregate(/* TODO */),
    Collation(CowStr),
    Conversion(/* TODO */),
    Database(/* TODO */),
    Domain(/* TODO */),
    EventTrigger(CowStr),
    ForeignDataWrapper(/* TODO */),
    ForeignServer(/* TODO */),
    Function(/* TODO */),
    Language(CowStr),
    LargeObject(/* TODO */),
    Operator(/* TODO */),
    OperatorClass(/* TODO */),
    OperatorFamily(/* TODO */),
    Procedure(/* TODO */),
    Publication(/* TODO */),
    Routine(/* TODO */),
    Schema(/* TODO */),
    StatisticExt(/* TODO */),
    Subscription(/* TODO */),
    Tablespace(/* TODO */),
    TextSearchConfiguration(/* TODO */),
    TextSearchDictionary(/* TODO */),
    Type(/* TODO */),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterOwnerStmt {
    target: AlterOwnerTarget,
    new_owner: RoleSpec,
}

impl AlterOwnerStmt {
    #[inline(always)]
    pub fn new(target: AlterOwnerTarget, new_owner: RoleSpec) -> Self {
        Self { target, new_owner }
    }

    #[inline(always)]
    pub fn target(&self) -> &AlterOwnerTarget {
        &self.target
    }

    #[inline(always)]
    pub fn new_owner(&self) -> &RoleSpec {
        &self.new_owner
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Literal(AstLiteral),
    SystemType(SystemType),
    TypeCast(/* ??? */),
    CheckPoint,
    ClosePortalStmt(OneOrAll),
    DeallocateStmt(OneOrAll),
    DiscardStmt(DiscardStmt),
    ListenStmt(CowStr),
    LoadStmt(String),
    ReassignOwnedStmt(ReassignOwnedStmt),
    VariableShowStmt(VariableShowStmt),
    UnlistenStmt(OneOrAll),
    TransactionStmt(TransactionStmt),
    NotifyStmt(NotifyStmt),
    AlterRoleStmt(AlterRoleStmt),
    RenameStmt(RenameStmt),
    AlterEventTrigStmt(AlterEventTrigStmt),
    AlterOwnerStmt(AlterOwnerStmt),
}

impl_from!(AlterEventTrigStmt for AstNode);
impl_from!(AlterOwnerStmt for AstNode);
impl_from!(AlterRoleStmt for AstNode);
impl_from!(AstLiteral for AstNode => Literal);
impl_from!(DiscardStmt for AstNode);
impl_from!(NotifyStmt for AstNode);
impl_from!(ReassignOwnedStmt for AstNode);
impl_from!(RenameStmt for AstNode);
impl_from!(SystemType for AstNode);
impl_from!(TransactionStmt for AstNode);
impl_from!(VariableShowStmt for AstNode);

use std::borrow::Cow;
