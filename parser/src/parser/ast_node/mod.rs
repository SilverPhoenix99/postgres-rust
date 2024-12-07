mod alter_database_stmt;
mod alter_default_privileges_stmt;
mod alter_event_trig_stmt;
mod alter_object_schema_stmt;
mod alter_owner_stmt;
mod alter_role_stmt;
mod binary_expr;
mod bool_expr;
mod case_expr;
mod create_database_stmt;
mod discard_stmt;
mod drop_behavior;
mod function_parameter;
mod grant_stmt;
mod indirection;
mod notify_stmt;
mod numeric_spec;
mod object_with_args;
mod one_or_all;
mod operator;
mod prepare_stmt;
mod qualified_operator;
mod range_var;
mod raw_stmt;
mod reassign_owned_stmt;
mod rename_stmt;
mod role_spec;
mod signed_number;
mod system_type;
mod transaction_stmt;
mod typecast_expr;
mod unary_expr;
mod unique_null_treatment;
mod unsigned_number;
mod variable_show_stmt;
mod xml;

pub use self::{
    alter_database_stmt::{AlterDatabaseStmt, AlterdbOption, AlterdbOptionKind},
    alter_default_privileges_stmt::AlterDefaultPrivilegesStmt,
    alter_event_trig_stmt::{AlterEventTrigStmt, EventTriggerState},
    alter_object_schema_stmt::{AlterObjectSchemaStmt, AlterObjectSchemaTarget},
    alter_owner_stmt::{AlterOwnerStmt, AlterOwnerTarget},
    alter_role_stmt::{AlterRoleAction, AlterRoleOption, AlterRoleStmt},
    binary_expr::BinaryExpr,
    bool_expr::BoolExpr,
    case_expr::{CaseExpr, CaseWhen},
    create_database_stmt::{CreateDatabaseStmt, CreatedbOption, CreatedbOptionKind, CreatedbOptionValue},
    discard_stmt::DiscardStmt,
    drop_behavior::DropBehavior,
    function_parameter::FunctionParameter,
    grant_stmt::GrantStmt,
    indirection::Indirection,
    notify_stmt::NotifyStmt,
    numeric_spec::NumericSpec,
    object_with_args::ObjectWithArgs,
    one_or_all::OneOrAll,
    operator::Operator,
    prepare_stmt::PrepareStmt,
    qualified_operator::QualifiedOperator,
    range_var::{RangeVar, RelationPersistence},
    raw_stmt::RawStmt,
    reassign_owned_stmt::ReassignOwnedStmt,
    rename_stmt::{RenameStmt, RenameTarget},
    role_spec::RoleSpec,
    signed_number::SignedNumber,
    system_type::{
        FuncType,
        FunctionParameterMode,
        IntervalRange,
        SetOf,
        Type,
        TypeModifiers,
        TypeName,
        TypeReference,
    },
    transaction_stmt::{IsolationLevel, TransactionMode, TransactionStmt},
    typecast_expr::TypecastExpr,
    unary_expr::UnaryExpr,
    unsigned_number::UnsignedNumber,
    variable_show_stmt::VariableShowStmt,
    xml::{XmlElement, XmlParse, XmlProcessingInstruction, XmlRoot},
};
pub(super) use unique_null_treatment::UniqueNullTreatment;

pub type BinaryOperands = Box<(ExprNode, ExprNode)>;
pub(super) type QualifiedName = Vec<Str>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AggregateWithArgtypes {
    // TODO
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionWithArgtypes {
    // TODO
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OperatorWithArgtypes {
    // TODO
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RelationExpr {
    // TODO
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AclOption {
    Schemas(Vec<Str>),
    Roles(Vec<RoleSpec>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpecificAccessPrivilege {
    AlterSystem,
    Create(Option<Vec<Str>>),
    References(Option<Vec<Str>>),
    Select(Option<Vec<Str>>),
    Named(Str, Option<Vec<Str>>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AccessPrivilege {
    All(Option<Vec<Str>>),
    Specific(Vec<SpecificAccessPrivilege>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AclTarget {
    Table,
    Function,
    Sequence,
    Type,
    Schema,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IndirectionExpr {
    Param {
        index: i32,
        indirection: Vec<Indirection>,
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExprNode {
    /* Constants */
    NullConst,
    StringConst(Box<str>),
    BinaryStringConst(Box<str>),
    HexStringConst(Box<str>),
    IntegerConst(i32),
    NumericConst { radix: NumberRadix, value: Box<str> },
    BooleanConst(bool),

    DefaultExpr,
    Typecast(Box<TypecastExpr>),
    CaseExpr(Box<CaseExpr>),

    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    /// `IS DISTINCT FROM`
    Distinct(BinaryOperands),
    /// `IS NOT DISTINCT FROM`
    NotDistinct(BinaryOperands),
    BoolExpr(BoolExpr),
    CurrentDate,
    CurrentTime { precision: Option<i32> },
    CurrentTimestamp { precision: Option<i32> },
    LocalTime { precision: Option<i32> },
    LocalTimestamp { precision: Option<i32> },
    CurrentRole,
    CurrentUser,
    SessionUser,
    SystemUser,
    User,
    CurrentCatalog,
    Indirection(Box<IndirectionExpr>),

    /* Xml operations */
    IsXmlDocument(Box<ExprNode>),
    XmlConcat(Vec<ExprNode>),
    XmlElement(XmlElement),
    XmlForest(Vec<ExprNode>),
    XmlParse(Box<XmlParse>),
    XmlProcessingInstruction(Box<XmlProcessingInstruction>),
    XmlRoot(Box<XmlRoot>),
}

impl_from!(box BinaryExpr for ExprNode);
impl_from!(box IndirectionExpr for ExprNode::Indirection);
impl_from!(box CaseExpr for ExprNode);
impl_from!(box TypecastExpr for ExprNode::Typecast);
impl_from!(box UnaryExpr for ExprNode);
impl_from!(box XmlParse for ExprNode);
impl_from!(box XmlProcessingInstruction for ExprNode);
impl_from!(box XmlRoot for ExprNode);
impl_from!(BoolExpr for ExprNode);
impl_from!(XmlElement for ExprNode);

impl From<UnsignedNumber> for ExprNode {
    fn from(value: UnsignedNumber) -> Self {
        use UnsignedNumber::*;
        match value {
            // SAFETY: `int` is originally parsed by `i32::from_str_radix()`, so `0 <= int <= i32::MAX`
            IntegerConst(int) => Self::IntegerConst(int.into()),
            NumericConst { value, radix } => Self::NumericConst { radix, value }
        }
    }
}

impl ExprNode {
    #[inline(always)]
    pub fn addition(left: Self, right: Self) -> Self {
        BinaryExpr::addition(left, right).into()
    }

    #[inline(always)]
    pub fn unary_plus(operand: Self) -> Self {
        UnaryExpr::new(Operator::Addition.into(), operand).into()
    }

    #[inline(always)]
    pub fn subtraction(left: Self, right: Self) -> Self {
        BinaryExpr::subtraction(left, right).into()
    }

    /// Aka `unary_minus`
    #[inline(always)]
    pub fn negation(operand: Self) -> Self {
        UnaryExpr::new(Operator::Subtraction.into(), operand).into()
    }

    #[inline(always)]
    pub fn multiplication(left: Self, right: Self) -> Self {
        BinaryExpr::multiplication(left, right).into()
    }

    #[inline(always)]
    pub fn division(left: Self, right: Self) -> Self {
        BinaryExpr::division(left, right).into()
    }

    #[inline(always)]
    pub fn modulo(left: Self, right: Self) -> Self {
        BinaryExpr::modulo(left, right).into()
    }

    #[inline(always)]
    pub fn exponentiation(left: Self, right: Self) -> Self {
        BinaryExpr::exponentiation(left, right).into()
    }

    #[inline(always)]
    pub fn less(left: Self, right: Self) -> Self {
        BinaryExpr::less(left, right).into()
    }

    #[inline(always)]
    pub fn greater(left: Self, right: Self) -> Self {
        BinaryExpr::greater(left, right).into()
    }

    #[inline(always)]
    pub fn equals(left: Self, right: Self) -> Self {
        BinaryExpr::equals(left, right).into()
    }

    #[inline(always)]
    pub fn greater_equals(left: Self, right: Self) -> Self {
        BinaryExpr::greater_equals(left, right).into()
    }

    #[inline(always)]
    pub fn less_equals(left: Self, right: Self) -> Self {
        BinaryExpr::less_equals(left, right).into()
    }

    #[inline(always)]
    pub fn not_equals(left: Self, right: Self) -> Self {
        BinaryExpr::not_equals(left, right).into()
    }

    #[inline(always)]
    pub fn distinct(left: Self, right: Self) -> Self {
        Self::Distinct(Box::new((left, right)))
    }

    #[inline(always)]
    pub fn not_distinct(left: Self, right: Self) -> Self {
        Self::NotDistinct(Box::new((left, right)))
    }

    #[inline(always)]
    pub fn is_xml_document(operand: Self) -> Self {
        Self::IsXmlDocument(Box::new(operand))
    }

    #[inline(always)]
    pub fn not(expr: Self) -> Self {
        BoolExpr::not(expr).into()
    }

    #[inline(always)]
    pub fn or(left: Self, right: Self) -> Self {
        BoolExpr::or(left, right).into()
    }

    #[inline(always)]
    pub fn and(left: Self, right: Self) -> Self {
        BoolExpr::and(left, right).into()
    }
}

use crate::NumberRadix;
use postgres_basics::impl_from;
use postgres_basics::Str;
