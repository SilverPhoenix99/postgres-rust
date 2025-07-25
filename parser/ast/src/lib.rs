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
mod column_ref;
mod comment_stmt;
mod constraints_set_mode;
mod constraints_set_stmt;
mod create_access_method_stmt;
mod create_cast_stmt;
mod create_conversion_stmt;
mod create_database_stmt;
mod create_role_stmt;
mod create_user_mapping_stmt;
mod discard_stmt;
mod drop_behavior;
mod frame_extent;
mod func_arg_expr;
mod func_args_kind;
mod func_call;
mod function_parameter;
mod function_with_args;
mod generic_option;
mod grant_option;
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
mod presence;
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
mod transaction_chain;
mod transaction_stmt;
mod transform;
mod typecast;
mod typecast_expr;
mod unary_expr;
mod unique_null_treatment;
mod unsigned_number;
mod utility_option;
mod value_or_default;
mod var_value;
mod variable_set_stmt;
mod variable_target;
mod window_definition;
mod window_frame;
mod xml;
mod zone_value;

pub use self::{
    aggregate_with_args::*,
    alter_database_set_stmt::*,
    alter_database_stmt::*,
    alter_default_privileges_stmt::*,
    alter_event_trig_stmt::*,
    alter_extension_contents_stmt::*,
    alter_extension_stmt::*,
    alter_function_stmt::*,
    alter_object_depends_stmt::*,
    alter_object_schema_stmt::*,
    alter_owner_stmt::*,
    alter_role_set_stmt::*,
    alter_role_stmt::*,
    alter_system_stmt::*,
    alter_user_mapping_stmt::*,
    binary_expr::*,
    bool_expr::*,
    boolean_or_string::*,
    case_expr::*,
    column_ref::*,
    comment_stmt::*,
    constraints_set_mode::*,
    constraints_set_stmt::*,
    create_access_method_stmt::*,
    create_cast_stmt::*,
    create_conversion_stmt::*,
    create_database_stmt::*,
    create_role_stmt::*,
    create_user_mapping_stmt::*,
    discard_stmt::*,
    drop_behavior::*,
    frame_extent::*,
    func_arg_expr::*,
    func_args_kind::*,
    func_call::*,
    function_parameter::*,
    function_with_args::*,
    generic_option::*,
    grant_option::*,
    grant_stmt::*,
    indirection::*,
    indirection_expr::*,
    notify_stmt::*,
    numeric_spec::*,
    one_or_all::*,
    one_or_both::*,
    operator::*,
    operator_with_args::*,
    over_clause::*,
    prepare_stmt::*,
    presence::*,
    privilege_target::*,
    qualified_operator::*,
    range_var::*,
    raw_stmt::*,
    reassign_owned_stmt::*,
    rename_stmt::*,
    role_spec::*,
    security_label_stmt::*,
    set_reset_clause::*,
    set_rest::*,
    set_rest_more::*,
    signed_number::*,
    sort_by::*,
    system_type::*,
    transaction_chain::*,
    transaction_stmt::*,
    transform::*,
    typecast::*,
    typecast_expr::*,
    unary_expr::*,
    unique_null_treatment::*,
    unsigned_number::*,
    utility_option::*,
    value_or_default::*,
    var_value::*,
    variable_set_stmt::*,
    variable_target::*,
    window_definition::*,
    window_frame::*,
    xml::*,
    zone_value::*,
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
    Create { columns: Option<Vec<Str>> },
    References { columns: Option<Vec<Str>> },
    Select { columns: Option<Vec<Str>> },
    Named {
        privilege: Str,
        columns: Option<Vec<Str>>
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AccessPrivilege {
    All { columns: Option<Vec<Str>> },
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
    Row(Option<Vec<ExprNode>>),

    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    BoolExpr(BoolExpr),
    /// `IS DISTINCT FROM`
    Distinct(BinaryOperands),
    /// `IS NOT DISTINCT FROM`
    NotDistinct(BinaryOperands),

    // TODO: Are these 2 the same?
    Indirection(Box<IndirectionExpr>),
    ColumnRef(Box<ColumnRef>),

    /* Function calls */
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
    CurrentSchema,
    FuncCall(Box<FuncCall>),
    CollationFor(Box<ExprNode>),

    /* Xml operations */
    IsXmlDocument(Box<ExprNode>),
    XmlConcat(Vec<ExprNode>),
    XmlElement(XmlElement),
    XmlForest(Vec<ExprNode>),
    XmlParse(Box<XmlParse>),
    XmlProcessingInstruction(Box<XmlProcessingInstruction>),
    XmlRoot(Box<XmlRoot>),
}

impl_from!(BoolExpr for ExprNode);
impl_from!(XmlElement for ExprNode);
impl_from!(box BinaryExpr for ExprNode);
impl_from!(box CaseExpr for ExprNode);
impl_from!(box ColumnRef for ExprNode);
impl_from!(box FuncCall for ExprNode);
impl_from!(box IndirectionExpr for ExprNode::Indirection);
impl_from!(box TypecastExpr for ExprNode::Typecast);
impl_from!(box UnaryExpr for ExprNode);
impl_from!(box XmlParse for ExprNode);
impl_from!(box XmlProcessingInstruction for ExprNode);
impl_from!(box XmlRoot for ExprNode);

impl From<UnsignedNumber> for ExprNode {
    fn from(value: UnsignedNumber) -> Self {
        match value {
            // SAFETY: `int` is originally parsed by `i32::from_str_radix()`, so `0 <= int <= i32::MAX`
            UnsignedNumber::IntegerConst(int) => Self::IntegerConst(int.into()),
            UnsignedNumber::NumericConst { value, radix } => Self::NumericConst { radix, value }
        }
    }
}

use pg_basics::impl_from;
use pg_basics::NumberRadix;
use pg_basics::Str;
