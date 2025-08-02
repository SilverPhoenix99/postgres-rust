pg_basics::reexport! { pub
    aggregate_with_args,
    alter_database_set_stmt,
    alter_database_stmt,
    alter_default_privileges_stmt,
    alter_event_trig_stmt,
    alter_extension_contents_stmt,
    alter_extension_stmt,
    alter_function_stmt,
    alter_object_depends_stmt,
    alter_object_schema_stmt,
    alter_owner_stmt,
    alter_role_set_stmt,
    alter_role_stmt,
    alter_system_stmt,
    alter_user_mapping_stmt,
    binary_expr,
    bool_expr,
    boolean_or_string,
    case_expr,
    column_ref,
    comment_stmt,
    constraints_set_mode,
    constraints_set_stmt,
    create_access_method_stmt,
    create_cast_stmt,
    create_conversion_stmt,
    create_database_stmt,
    create_role_stmt,
    create_user_mapping_stmt,
    discard_stmt,
    drop_behavior,
    extract_expr,
    frame_extent,
    func_args_kind,
    func_call,
    function_parameter,
    function_with_args,
    generic_option,
    grant_option,
    grant_stmt,
    indirection,
    indirection_expr,
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
    json_value_expr,
    json_value_func,
    json_wrapper_behavior,
    named_value,
    normalize_func,
    notify_stmt,
    numeric_spec,
    one_or_all,
    one_or_both,
    operator,
    operator_with_args,
    over_clause,
    overlay_func,
    position_func,
    prepare_stmt,
    presence,
    privilege_target,
    qualified_operator,
    range_var,
    raw_stmt,
    reassign_owned_stmt,
    relation_expr,
    rename_stmt,
    role_spec,
    security_label_stmt,
    set_reset_clause,
    set_rest,
    set_rest_more,
    signed_number,
    sort_by,
    substring_func,
    system_type,
    transaction_chain,
    transaction_stmt,
    transform,
    trim_func,
    typecast,
    typecast_expr,
    unary_expr,
    unicode_normal_form,
    unique_null_treatment,
    unsigned_number,
    utility_option,
    value_or_default,
    var_value,
    variable_set_stmt,
    variable_target,
    window_definition,
    window_frame,
    xml_element,
    xml_exists,
    xml_node_kind,
    xml_parse,
    xml_processing_instruction,
    xml_root,
    xml_serialize,
    zone_value,
}

pub type BinaryOperands = Box<(ExprNode, ExprNode)>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AddDrop {
    Add,
    Drop,
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
    NullIf(BinaryOperands),
    Coalesce(Vec<ExprNode>),
    Greatest(Vec<ExprNode>),
    Least(Vec<ExprNode>),
    Treat(Box<TypecastExpr>),
    MergeAction,
    JsonScalar(Box<ExprNode>),
    JsonExists(Box<JsonExistsExpr>),
    JsonQuery(Box<JsonQueryExpr>),
    JsonSerialize(Box<JsonSerializeExpr>),
    JsonValue(Box<JsonValueFunc>),

    // TODO: Are these 2 the same?
    Indirection(Box<IndirectionExpr>),
    ColumnRef(ColumnRef),

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
    CollationForFunc(Box<ExprNode>),
    GroupingFunc(Vec<ExprNode>),
    ExtractFunc(Box<ExtractFunc>),
    NormalizeFunc(Box<NormalizeFunc>),
    PositionFunc(Box<PositionFunc>),
    TrimFunc(TrimFunc),
    JsonArrayAgg(Box<JsonArrayAggExpr>),
    JsonObject(JsonObjectExpr),
    JsonObjectAgg(Box<JsonObjectAggExpr>),
    OverlayFunc(Box<OverlayFunc>),
    SubstringFunc(Box<SubstringFunc>),

    /* Xml operations */
    XmlConcat(Vec<ExprNode>),
    XmlElement(XmlElement),
    XmlExists(Box<XmlExists>),
    XmlForest(Vec<NamedValue>),
    XmlParse(Box<XmlParse>),
    XmlProcessingInstruction(Box<XmlProcessingInstruction>),
    XmlRoot(Box<XmlRoot>),
    XmlSerialize(Box<XmlSerialize>),
}

impl_from!(BoolExpr for ExprNode);
impl_from!(ColumnRef for ExprNode);
impl_from!(JsonObjectExpr for ExprNode::JsonObject);
impl_from!(TrimFunc for ExprNode);
impl_from!(XmlElement for ExprNode);
impl_from!(box BinaryExpr for ExprNode);
impl_from!(box CaseExpr for ExprNode);
impl_from!(box ExtractFunc for ExprNode);
impl_from!(box FuncCall for ExprNode);
impl_from!(box IndirectionExpr for ExprNode::Indirection);
impl_from!(box JsonArrayAggExpr for ExprNode::JsonArrayAgg);
impl_from!(box JsonExistsExpr for ExprNode::JsonExists);
impl_from!(box JsonObjectAggExpr for ExprNode::JsonObjectAgg);
impl_from!(box JsonQueryExpr for ExprNode::JsonQuery);
impl_from!(box JsonSerializeExpr for ExprNode::JsonSerialize);
impl_from!(box JsonValueFunc for ExprNode::JsonValue);
impl_from!(box NormalizeFunc for ExprNode);
impl_from!(box OverlayFunc for ExprNode);
impl_from!(box PositionFunc for ExprNode);
impl_from!(box SubstringFunc for ExprNode);
impl_from!(box TypecastExpr for ExprNode::Typecast);
impl_from!(box UnaryExpr for ExprNode);
impl_from!(box XmlExists for ExprNode);
impl_from!(box XmlParse for ExprNode);
impl_from!(box XmlProcessingInstruction for ExprNode);
impl_from!(box XmlRoot for ExprNode);
impl_from!(box XmlSerialize for ExprNode);

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
