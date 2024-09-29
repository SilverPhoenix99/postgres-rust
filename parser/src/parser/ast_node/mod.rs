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
    pub fn new(roles: Vec<RoleSpec>, new_role: RoleSpec) -> Self {
        Self { roles, new_role }
    }

    pub fn roles(&self) -> &Vec<RoleSpec> {
        &self.roles
    }

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
}

impl From<AstLiteral> for AstNode {
    fn from(value: AstLiteral) -> Self {
        Self::Literal(value)
    }
}

impl From<SystemType> for AstNode {
    fn from(value: SystemType) -> Self {
        Self::SystemType(value)
    }
}

impl From<ClosePortalStmt> for AstNode {
    fn from(value: ClosePortalStmt) -> Self {
        Self::ClosePortalStmt(value)
    }
}

impl From<DeallocateStmt> for AstNode {
    fn from(value: DeallocateStmt) -> Self {
        Self::DeallocateStmt(value)
    }
}

impl From<DiscardStmt> for AstNode {
    fn from(value: DiscardStmt) -> Self {
        Self::DiscardStmt(value)
    }
}

impl From<ReassignOwnedStmt> for AstNode {
    fn from(value: ReassignOwnedStmt) -> Self {
        Self::ReassignOwnedStmt(value)
    }
}

impl From<VariableShowStmt> for AstNode {
    fn from(value: VariableShowStmt) -> Self {
        Self::VariableShowStmt(value)
    }
}

impl From<UnlistenStmt> for AstNode {
    fn from(value: UnlistenStmt) -> Self {
        Self::UnlistenStmt(value)
    }
}

use bitvec::boxed::BitBox;
use std::borrow::Cow;
