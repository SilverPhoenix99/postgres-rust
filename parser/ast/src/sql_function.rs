/// Special expressions that might not look like function calls,
/// but will likely call pre-defined functions in Postgres.
#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum SqlFunction {

    /// `COALESCE( expr... )`
    Coalesce(Vec<ExprNode>),

    /// `COLLATION FOR( expr )`
    CollationFor(ExprNode),

    /// `CURRENT_CATALOG`
    CurrentCatalog,

    /// `CURRENT_SCHEMA`
    CurrentSchema,

    /// `GREATEST( expr... )`
    Greatest(Vec<ExprNode>),

    /// `LEAST( expr... )`
    Least(Vec<ExprNode>),

    /// `MERGE_ACTION()`
    MergeAction,

    /// `NULLIF( expr , expr )`
    NullIf(ExprNode, ExprNode),

    /// `TREAT( expr AS Type )`
    Treat(TypecastExpr),

    /// Typecasts:
    /// * `'1'::int`
    /// * `int '1'`
    /// * `CAST('1' as int)`
    #[from]
    Typecast(TypecastExpr),

    /*
        String functions
    */

    /// `NORMALIZE( expr , unicode_normal_form )`
    #[from]
    Normalize(NormalizeFunc),

    /// `OVERLAY( ... )`
    #[from]
    Overlay(OverlayFunc),

    /// `POSITION( expr IN expr )`
    #[from]
    Position(PositionFunc),

    /// `SUBSTRING( ... )`
    #[from]
    Substring(SubstringFunc),

    /// `TRIM( ... )`
    #[from]
    Trim(TrimFunc),

    /*
        Time functions
    */

    /// `CURRENT_DATE`
    CurrentDate,

    /// * `CURRENT_TIME`
    /// * `CURRENT_TIME( ICONST )`
    CurrentTime { precision: Option<i32> },

    /// * `CURRENT_TIMESTAMP`
    /// * `CURRENT_TIMESTAMP( ICONST )`
    CurrentTimestamp { precision: Option<i32> },

    /// * `LOCALTIME`
    /// * `LOCALTIME( ICONST )`
    LocalTime { precision: Option<i32> },

    /// * `LOCALTIMESTAMP`
    /// * `LOCALTIMESTAMP( ICONST )`
    LocalTimestamp { precision: Option<i32> },

    /// `EXTRACT( ... )`
    #[from]
    Extract(ExtractFunc),

    /*
        Role functions
    */

    /// `CURRENT_ROLE`
    CurrentRole,

    /// `CURRENT_USER`
    CurrentUser,

    /// `SESSION_USER`
    SessionUser,

    /// `SYSTEM_USER`
    SystemUser,

    /// `USER`
    User,

    /*
        JSON functions
    */

    /// `JSON( ... )`
    #[from]
    Json(JsonFunc),

    /// `JSON_ARRAY( ... )`
    #[from]
    JsonArray(JsonArrayConstructor),

    /// `JSON_ARRAY( select_stmt ... )`
    #[from]
    JsonArrayQuery(JsonArrayQueryConstructor),

    /// `JSON_ARRAY()`
    JsonArrayEmpty(Option<JsonOutput>),

    /// `JSON_ARRAYAGG( ... )`
    #[from]
    JsonArrayAgg(JsonArrayAgg),

    /// `JSON_EXISTS( ... )`
    #[from]
    JsonExists(JsonExistsExpr),

    /// `JSON_OBJECT( ... )`
    #[from]
    JsonObject(JsonObjectExpr),

    /// `JSON_OBJECTAGG( ... )`
    #[from]
    JsonObjectAgg(JsonObjectAgg),

    /// `JSON_QUERY( ... )`
    #[from]
    JsonQuery(JsonQueryExpr),

    /// `JSON_SCALAR( expr )`
    #[from]
    JsonScalar(ExprNode),

    /// `JSON_SERIALIZE( ... )`
    #[from]
    JsonSerialize(JsonSerializeExpr),

    /// `JSON_VALUE( ... )`
    #[from]
    JsonValue(JsonValueFunc),

    /*
        XML functions
    */

    /// `XMLCONCAT( expr... )`
    XmlConcat(Vec<ExprNode>),

    /// `XMLELEMENT( ... )`
    #[from]
    XmlElement(XmlElement),

    /// `XMLEXISTS( ... )`
    #[from]
    XmlExists(XmlExists),

    /// `XMLFOREST( ... )`
    XmlForest(Vec<NamedValue>),

    /// `XMLPARSE( ... )`
    #[from]
    XmlParse(XmlParse),

    /// `XMLPI( ... )`
    #[from]
    XmlProcessingInstruction(XmlProcessingInstruction),

    /// `XMLROOT( ... )`
    #[from]
    XmlRoot(XmlRoot),

    /// `XMLSERIALIZE( ... )`
    #[from]
    XmlSerialize(XmlSerialize),
}

use crate::ExprNode;
use crate::ExtractFunc;
use crate::JsonArrayAgg;
use crate::JsonArrayConstructor;
use crate::JsonArrayQueryConstructor;
use crate::JsonExistsExpr;
use crate::JsonFunc;
use crate::JsonObjectAgg;
use crate::JsonObjectExpr;
use crate::JsonOutput;
use crate::JsonQueryExpr;
use crate::JsonSerializeExpr;
use crate::JsonValueFunc;
use crate::NamedValue;
use crate::NormalizeFunc;
use crate::OverlayFunc;
use crate::PositionFunc;
use crate::SubstringFunc;
use crate::TrimFunc;
use crate::TypecastExpr;
use crate::XmlElement;
use crate::XmlExists;
use crate::XmlParse;
use crate::XmlProcessingInstruction;
use crate::XmlRoot;
use crate::XmlSerialize;
use derive_more::From;
