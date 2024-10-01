pub mod numeric_spec;

pub use self::numeric_spec::NumericSpec;

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
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(CowStr),
}

impl RoleSpec {
    pub fn into_role_id(self) -> Result<CowStr, ParserErrorKind> {
        match self {
            Self::Name(role) => Ok(role),
            Self::Public => Err(ReservedRoleSpec("public")),
            Self::CurrentRole => Err(ForbiddenRoleSpec("CURRENT_ROLE")),
            Self::CurrentUser => Err(ForbiddenRoleSpec("CURRENT_USER")),
            Self::SessionUser => Err(ForbiddenRoleSpec("SESSION_USER")),
        }
    }
}

/// Redundant enum, to avoid using `unreachable!()`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum CharacterSystemType {
    Varchar,
    Bpchar
}

#[derive(Debug, Clone, PartialEq)]
pub enum SystemType {
    // Character types:
    // If limited, the maximum is 10MB == 10,485,760
    // see https://www.postgresql.org/docs/current/datatype-character.html
    Varchar(Option<i32>),
    /// Blank-Padded Character string
    Bpchar(Option<i32>),
    // Bit string types:
    Bit(Vec<AstNode>),
    Varbit(Vec<AstNode>),
    // Numeric types:
    Bool,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Numeric(Vec<AstNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstLiteral {
    StringLiteral(String),
    BitStringLiteral(BitBox),
    IntegerLiteral(i32),
    FloatLiteral(f64),
    NumericLiteral(String), // TODO: Replace with some kind of BigDecimal
    BooleanLiteral(bool),
    NullLiteral,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClosePortalStmt {
    All,
    Name(CowStr),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeallocateStmt {
    All,
    Name(CowStr)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DiscardStmt {
    All,
    Plans,
    Sequences,
    Temporary,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReassignOwnedStmt {
    roles: Vec<RoleSpec>,
    new_role: RoleSpec,
}

impl ReassignOwnedStmt {
    #[inline(always)]
    pub fn new(roles: Vec<RoleSpec>, new_role: RoleSpec) -> Self {
        Self { roles, new_role }
    }

    #[inline(always)]
    pub fn roles(&self) -> &Vec<RoleSpec> {
        &self.roles
    }

    #[inline(always)]
    pub fn new_role(&self) -> &RoleSpec {
        &self.new_role
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VariableShowStmt {
    All,
    SessionAuthorization,
    TransactionIsolation,
    TimeZone,
    // Name, possibly qualified, separated by dots
    Name(Vec<Cow<'static, str>>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnlistenStmt {
    All,
    Name(CowStr),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TransactionMode {
    IsolationLevel(IsolationLevel),
    ReadOnly,
    ReadWrite,
    Deferrable,
    NotDeferrable,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TransactionStmt {
    Begin(Vec<TransactionMode>),
    /// Semantically identical to `BEGIN`.
    Start(Vec<TransactionMode>),
    Commit { chain: bool },
    CommitPrepared(String),
    Savepoint(CowStr),
    Release(CowStr),
    Prepare(String),
    Rollback { chain: bool },
    RollbackTo(CowStr),
    RollbackPrepared(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotifyStmt {
    condition_name: CowStr,
    payload: Option<String>
}

impl NotifyStmt {
    #[inline(always)]
    pub fn new(condition_name: CowStr) -> Self {
        Self { condition_name, payload: None }
    }

    #[inline(always)]
    pub fn with_payload(condition_name: CowStr, payload: String) -> Self {
        Self { condition_name, payload: Some(payload) }
    }

    #[inline(always)]
    pub fn condition_name(&self) -> &CowStr {
        &self.condition_name
    }

    #[inline(always)]
    pub fn payload(&self) -> &Option<String> {
        &self.payload
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AlterRoleAction {
    Add,
    Remove,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterRoleOption {
    RoleMembers(Vec<RoleSpec>),
    Password(Option<String>),
    Inherit(bool),
    ConnectionLimit(i32),
    ValidUntil(String),
    SuperUser(bool),
    CreateRole(bool),
    Replication(bool),
    CreateDatabase(bool),
    CanLogin(bool),
    BypassRls(bool),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterRoleStmt {
    role: RoleSpec,
    action: AlterRoleAction,
    options: Vec<AlterRoleOption>,
}

impl AlterRoleStmt {
    #[inline(always)]
    pub fn new(role: RoleSpec, action: AlterRoleAction, options: Vec<AlterRoleOption>) -> Self {
        Self { role, action, options }
    }

    #[inline(always)]
    pub fn role(&self) -> &RoleSpec {
        &self.role
    }

    #[inline(always)]
    pub fn action(&self) -> AlterRoleAction {
        self.action
    }

    #[inline(always)]
    pub fn options(&self) -> &Vec<AlterRoleOption> {
        &self.options
    }

    #[inline(always)]
    pub fn add(&self) -> bool {
        self.action == AlterRoleAction::Add
    }

    #[inline(always)]
    pub fn remove(&self) -> bool {
        self.action == AlterRoleAction::Remove
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RenameStmt {
    Role {
        sub_name: CowStr,
        new_name: CowStr,
    },
    EventTrigger {
        trigger: CowStr,
        new_name: CowStr,
    }
    // TODO
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EventTriggerState {
    FiresOnReplica,
    FiresOnOrigin,
    FiresAlways,
    Disabled,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterOwnerStmt {
    EventTrigger {
        trigger: CowStr,
        new_owner: RoleSpec,
    },
    // TODO
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Literal(AstLiteral),
    SystemType(SystemType),
    TypeCast(/* ??? */),
    CheckPoint,
    ClosePortalStmt(ClosePortalStmt),
    DeallocateStmt(DeallocateStmt),
    DiscardStmt(DiscardStmt),
    ListenStmt(CowStr),
    LoadStmt(String),
    ReassignOwnedStmt(ReassignOwnedStmt),
    VariableShowStmt(VariableShowStmt),
    UnlistenStmt(UnlistenStmt),
    TransactionStmt(TransactionStmt),
    NotifyStmt(NotifyStmt),
    AlterRoleStmt(AlterRoleStmt),
    RenameStmt(RenameStmt),
    AlterEventTrigStmt {
        trigger: CowStr,
        state: EventTriggerState
    },
    AlterOwnerStmt(AlterOwnerStmt)
}

impl_from!(AlterOwnerStmt for AstNode);
impl_from!(AlterRoleStmt for AstNode);
impl_from!(AstLiteral for AstNode => Literal);
impl_from!(ClosePortalStmt for AstNode);
impl_from!(DeallocateStmt for AstNode);
impl_from!(DiscardStmt for AstNode);
impl_from!(NotifyStmt for AstNode);
impl_from!(ReassignOwnedStmt for AstNode);
impl_from!(RenameStmt for AstNode);
impl_from!(SystemType for AstNode);
impl_from!(TransactionStmt for AstNode);
impl_from!(UnlistenStmt for AstNode);
impl_from!(VariableShowStmt for AstNode);

use crate::parser::ParserErrorKind;
use crate::parser::ParserErrorKind::{ForbiddenRoleSpec, ReservedRoleSpec};
use bitvec::boxed::BitBox;
use std::borrow::Cow;
