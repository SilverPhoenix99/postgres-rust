mod aggregate_with_args;
mod alter_database_set_stmt;
mod alter_database_stmt;
mod alter_default_privileges_stmt;
mod alter_event_trig_stmt;
mod alter_extension_contents_stmt;
mod alter_extension_stmt;
mod alter_function_stmt;
mod alter_object_depends_stmt;
mod alter_object_schema_stmt;
mod alter_owner_stmt;
mod alter_role_set_stmt;
mod alter_role_stmt;
mod alter_system_stmt;
mod alter_user_mapping_stmt;
mod binary_expr;
mod bool_expr;
mod boolean_or_string;
mod case_expr;
mod comment_stmt;
mod constraints_set_mode;
mod constraints_set_stmt;
mod create_access_method_stmt;
mod create_cast_stmt;
mod create_conversion_stmt;
mod create_database_stmt;
mod create_role_stmt;
mod create_user_mapping_stmt;
mod default_or_value;
mod discard_stmt;
mod drop_behavior;
mod frame_extent;
mod func_arg_expr;
mod function_parameter;
mod function_with_args;
mod generic_option;
mod grant_stmt;
mod indirection;
mod indirection_expr;
mod notify_stmt;
mod numeric_spec;
mod one_or_all;
mod one_or_both;
mod operator;
mod operator_with_args;
mod over_clause;
mod prepare_stmt;
mod privilege_target;
mod qualified_operator;
mod range_var;
mod raw_stmt;
mod reassign_owned_stmt;
mod rename_stmt;
mod role_spec;
mod security_label_stmt;
mod set_reset_clause;
mod set_rest;
mod set_rest_more;
mod signed_number;
mod sort_by;
mod system_type;
mod transaction_stmt;
mod transform;
mod typecast;
mod typecast_expr;
mod unary_expr;
mod unique_null_treatment;
mod unsigned_number;
mod var_value;
mod variable_set_stmt;
mod variable_target;
mod window_definition;
mod window_frame;
mod xml;
mod zone_value;

pub use self::{
    aggregate_with_args::AggregateWithArgs,
    alter_database_set_stmt::AlterDatabaseSetStmt,
    alter_database_stmt::{AlterDatabaseStmt, AlterdbOption, AlterdbOptionKind},
    alter_default_privileges_stmt::AlterDefaultPrivilegesStmt,
    alter_event_trig_stmt::{AlterEventTrigStmt, EventTriggerState},
    alter_extension_contents_stmt::{AlterExtensionContentsStmt, AlterExtensionContentsTarget},
    alter_extension_stmt::AlterExtensionStmt,
    alter_function_stmt::{AlterFunctionKind, AlterFunctionOption, AlterFunctionStmt, Volatility},
    alter_object_depends_stmt::{AlterObjectDependsStmt, AlterObjectDependsTarget},
    alter_object_schema_stmt::{AlterObjectSchemaStmt, AlterObjectSchemaTarget},
    alter_owner_stmt::{AlterOwnerStmt, AlterOwnerTarget},
    alter_role_set_stmt::AlterRoleSetStmt,
    alter_role_stmt::{AlterRoleOption, AlterRoleStmt},
    alter_system_stmt::AlterSystemStmt,
    alter_user_mapping_stmt::AlterUserMappingStmt,
    binary_expr::BinaryExpr,
    bool_expr::BoolExpr,
    boolean_or_string::BooleanOrString,
    case_expr::{CaseExpr, CaseWhen},
    comment_stmt::{CommentStmt, CommentTarget},
    constraints_set_mode::ConstraintsSetMode,
    constraints_set_stmt::ConstraintsSetStmt,
    create_access_method_stmt::{AccessMethodKind, CreateAccessMethodStmt},
    create_cast_stmt::{CastConversion, CoercionContext, CreateCastStmt},
    create_conversion_stmt::CreateConversionStmt,
    create_database_stmt::{CreateDatabaseStmt, CreatedbOption, CreatedbOptionKind, CreatedbOptionValue},
    create_role_stmt::{CreateRoleOption, CreateRoleStmt, RoleKind},
    create_user_mapping_stmt::CreateUserMappingStmt,
    default_or_value::ValueOrDefault,
    discard_stmt::DiscardStmt,
    drop_behavior::DropBehavior,
    frame_extent::{CurrentRowEnd, FollowingEnd, FrameExtent, PrecedingEnd},
    func_arg_expr::FuncArgExpr,
    function_parameter::FunctionParameter,
    function_with_args::FunctionWithArgs,
    generic_option::{GenericOption, GenericOptionKind},
    grant_stmt::GrantStmt,
    indirection::Indirection,
    indirection_expr::IndirectionExpr,
    notify_stmt::NotifyStmt,
    numeric_spec::NumericSpec,
    one_or_all::OneOrAll,
    one_or_both::OneOrBoth,
    operator::Operator,
    operator_with_args::OperatorWithArgs,
    over_clause::OverClause,
    prepare_stmt::PrepareStmt,
    privilege_target::PrivilegeTarget,
    qualified_operator::QualifiedOperator,
    range_var::{RangeVar, RelationName, RelationPersistence, SchemaName},
    raw_stmt::RawStmt,
    reassign_owned_stmt::ReassignOwnedStmt,
    rename_stmt::{RenameStmt, RenameTarget},
    role_spec::RoleSpec,
    security_label_stmt::{SecurityLabelStmt, SecurityLabelTarget},
    set_reset_clause::SetResetClause,
    set_rest::SetRest,
    set_rest_more::SetRestMore,
    signed_number::SignedNumber,
    sort_by::{SortBy, SortDirection, SortNulls},
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
    transform::Transform,
    typecast::Typecast,
    typecast_expr::TypecastExpr,
    unary_expr::UnaryExpr,
    unique_null_treatment::UniqueNullTreatment,
    unsigned_number::UnsignedNumber,
    var_value::VarValue,
    variable_set_stmt::VariableSetStmt,
    variable_target::VariableTarget,
    window_definition::WindowDefinition,
    window_frame::{WindowExclusion, WindowFrame, WindowFrameKind},
    xml::{XmlElement, XmlNodeKind, XmlParse, XmlProcessingInstruction, XmlRoot},
    zone_value::ZoneValue,
};

pub type BinaryOperands = Box<(ExprNode, ExprNode)>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AddDrop {
    Add,
    Drop,
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
pub enum PrivilegeDefaultsTarget {
    Functions,
    LargeObjects,
    Schemas,
    Sequences,
    Tables,
    Types,
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
    ParamRef { index: i32 },

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

use pg_basics::impl_from;
use pg_basics::NumberRadix;
use pg_basics::Str;
