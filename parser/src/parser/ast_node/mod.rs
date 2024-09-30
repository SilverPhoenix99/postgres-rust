pub mod numeric_spec;

pub use self::numeric_spec::NumericSpec;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Cow<'static, str>),
}

impl RoleSpec {
    pub fn into_role_id(self) -> Result<Cow<'static, str>, ParserErrorKind> {
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
    Name(Cow<'static, str>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeallocateStmt {
    All,
    Name(Cow<'static, str>)
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
    Name(Cow<'static, str>),
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
    Savepoint(Cow<'static, str>),
    Release(Cow<'static, str>),
    Prepare(String),
    Rollback { chain: bool },
    RollbackTo(Cow<'static, str>),
    RollbackPrepared(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotifyStmt {
    condition_name: Cow<'static, str>,
    payload: Option<String>
}

impl NotifyStmt {
    #[inline(always)]
    pub fn new(condition_name: Cow<'static, str>) -> Self {
        Self { condition_name, payload: None }
    }

    #[inline(always)]
    pub fn with_payload(condition_name: Cow<'static, str>, payload: String) -> Self {
        Self { condition_name, payload: Some(payload) }
    }

    #[inline(always)]
    pub fn condition_name(&self) -> &Cow<'static, str> {
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
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RenameStmt {
    Role {
        sub_name: Cow<'static, str>,
        new_name: Cow<'static, str>,
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
    ListenStmt(Cow<'static, str>),
    LoadStmt(String),
    ReassignOwnedStmt(ReassignOwnedStmt),
    VariableShowStmt(VariableShowStmt),
    UnlistenStmt(UnlistenStmt),
    TransactionStmt(TransactionStmt),
    NotifyStmt(NotifyStmt),
    AlterRoleStmt(AlterRoleStmt),
    RenameStmt(RenameStmt),
}

impl From<AstLiteral> for AstNode {
    #[inline(always)]
    fn from(value: AstLiteral) -> Self {
        Self::Literal(value)
    }
}

impl From<SystemType> for AstNode {
    #[inline(always)]
    fn from(value: SystemType) -> Self {
        Self::SystemType(value)
    }
}

impl From<ClosePortalStmt> for AstNode {
    #[inline(always)]
    fn from(value: ClosePortalStmt) -> Self {
        Self::ClosePortalStmt(value)
    }
}

impl From<DeallocateStmt> for AstNode {
    #[inline(always)]
    fn from(value: DeallocateStmt) -> Self {
        Self::DeallocateStmt(value)
    }
}

impl From<DiscardStmt> for AstNode {
    #[inline(always)]
    fn from(value: DiscardStmt) -> Self {
        Self::DiscardStmt(value)
    }
}

impl From<ReassignOwnedStmt> for AstNode {
    #[inline(always)]
    fn from(value: ReassignOwnedStmt) -> Self {
        Self::ReassignOwnedStmt(value)
    }
}

impl From<VariableShowStmt> for AstNode {
    #[inline(always)]
    fn from(value: VariableShowStmt) -> Self {
        Self::VariableShowStmt(value)
    }
}

impl From<UnlistenStmt> for AstNode {
    #[inline(always)]
    fn from(value: UnlistenStmt) -> Self {
        Self::UnlistenStmt(value)
    }
}

impl From<TransactionStmt> for AstNode {
    #[inline(always)]
    fn from(value: TransactionStmt) -> Self {
        Self::TransactionStmt(value)
    }
}

impl From<NotifyStmt> for AstNode {
    #[inline(always)]
    fn from(value: NotifyStmt) -> Self {
        Self::NotifyStmt(value)
    }
}

impl From<AlterRoleStmt> for AstNode {
    #[inline(always)]
    fn from(value: AlterRoleStmt) -> Self {
        Self::AlterRoleStmt(value)
    }
}

impl From<RenameStmt> for AstNode {
    #[inline(always)]
    fn from(value: RenameStmt) -> Self {
        Self::RenameStmt(value)
    }
}

use crate::parser::ParserErrorKind;
use crate::parser::ParserErrorKind::{ForbiddenRoleSpec, ReservedRoleSpec};
use bitvec::boxed::BitBox;
use std::borrow::Cow;
