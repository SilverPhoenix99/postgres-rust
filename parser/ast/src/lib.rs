pg_basics::reexport! { pub
    aggregate_with_args,
    alias,
    alter_default_privileges_stmt,
    alter_event_trig_stmt,
    alter_extension_contents_stmt,
    alter_extension_stmt,
    alter_function_stmt,
    alter_object_depends_stmt,
    alter_object_schema_stmt,
    alter_owner_stmt,
    alter_system_stmt,
    binary_expr,
    bool_expr,
    case_expr,
    column_ref,
    comment_stmt,
    constraints_set_mode,
    constraints_set_stmt,
    create_access_method_stmt,
    create_cast_stmt,
    create_conversion_stmt,
    discard_stmt,
    extract_expr,
    frame_extent,
    func_alias,
    func_args_kind,
    func_call,
    func_call_expr,
    func_expr_windowless,
    function_parameter,
    function_with_args,
    grant_option,
    grant_stmt,
    indirection,
    indirection_expr,
    json,
    json_array_agg,
    json_array_agg_expr,
    json_behavior,
    json_exists,
    json_format,
    json_key_value,
    json_object,
    json_object_agg,
    json_object_agg_expr,
    json_output,
    json_query,
    json_quotes,
    json_serialize,
    json_table,
    json_table_column_definition,
    json_table_path_spec,
    json_value_expr,
    json_value_func,
    json_wrapper_behavior,
    named_value,
    normalize_func,
    notify_stmt,
    numeric_spec,
    one_or_both,
    operator_with_args,
    over_clause,
    overlay_func,
    position_func,
    prepare_stmt,
    privilege_target,
    range_function,
    range_var,
    raw_stmt,
    reassign_owned_stmt,
    rename_stmt,
    security_label_stmt,
    simple_column_definition,
    sort_by,
    sql_function,
    substring_func,
    system_type,
    transform,
    trim_func,
    typecast,
    typecast_expr,
    unary_expr,
    unicode_normal_form,
    unique_null_treatment,
    variable_set_stmt,
    window_definition,
    window_frame,
    xml_element,
    xml_exists,
    xml_parse,
    xml_processing_instruction,
    xml_root,
    xml_serialize,
    xmltable,
}

pub type BinaryOperands = Box<(ExprNode, ExprNode)>;

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

#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum ExprNode {
    /* Constants */
    NullConst,
    StringConst(Box<str>),
    BinaryStringConst(Box<str>),
    HexStringConst(Box<str>),
    IntegerConst(i32),
    NumericConst { value: Box<str>, radix: NumberRadix },
    BooleanConst(bool),

    DefaultExpr,
    #[from(CaseExpr)]
    CaseExpr(Box<CaseExpr>),
    ParamRef { index: i32 },
    Row(Option<Vec<ExprNode>>),

    /// String constant type cast.
    ///
    /// (e.g.: `int '1'`)
    #[from(StringTypecastExpr)]
    StringTypecast(Box<StringTypecastExpr>),

    #[from(BinaryExpr)]
    BinaryExpr(Box<BinaryExpr>),
    #[from(UnaryExpr)]
    UnaryExpr(Box<UnaryExpr>),
    #[from]
    BoolExpr(BoolExpr),
    #[from(FuncCallExpr)]
    FuncCallExpr(Box<FuncCallExpr>),
    #[from(JsonArrayAggExpr)]
    JsonArrayAggExpr(Box<JsonArrayAggExpr>),
    #[from(JsonObjectAggExpr)]
    JsonObjectAggExpr(Box<JsonObjectAggExpr>),

    /// `IS DISTINCT FROM`
    Distinct(BinaryOperands),
    /// `IS NOT DISTINCT FROM`
    NotDistinct(BinaryOperands),

    #[from(IndirectionExpr)]
    Indirection(Box<IndirectionExpr>),
    #[from]
    ColumnRef(ColumnRef),

    /* Function calls */
    GroupingFunc(Vec<ExprNode>),
    #[from(FuncCall)]
    FuncCall(Box<FuncCall>),
    #[from(SqlFunction)]
    SqlFunction(Box<SqlFunction>),
}

impl From<UnsignedNumber> for ExprNode {
    fn from(value: UnsignedNumber) -> Self {
        match value {
            // SAFETY: `int` is originally parsed by `i32::from_str_radix()`, so `0 <= int <= i32::MAX`
            UnsignedNumber::IntegerConst(int) => Self::IntegerConst(int.into()),
            UnsignedNumber::NumericConst { value, radix } => Self::NumericConst { radix, value }
        }
    }
}

use derive_more::From;
use pg_basics::NumberRadix;
use pg_basics::Str;
use pg_basics::UnsignedNumber;
use pg_sink_ast::RoleSpec;
