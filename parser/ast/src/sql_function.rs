#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SqlFunction {

    Coalesce(Vec<ExprNode>),
    CollationFor(ExprNode),
    CurrentCatalog,
    CurrentSchema,
    Greatest(Vec<ExprNode>),
    Least(Vec<ExprNode>),
    MergeAction,
    NullIf(ExprNode, ExprNode),
    Treat(TypecastExpr),

    /// SQL function-like format, or double-colon format
    ///
    /// (e.g.`CAST(1 to text)` or `1::text`)
    Typecast(TypecastExpr),

    // String functions
    Normalize(NormalizeFunc),
    Overlay(OverlayFunc),
    Position(PositionFunc),
    Substring(SubstringFunc),
    Trim(TrimFunc),

    // Time functions
    CurrentDate,
    CurrentTime { precision: Option<i32> },
    CurrentTimestamp { precision: Option<i32> },
    LocalTime { precision: Option<i32> },
    LocalTimestamp { precision: Option<i32> },
    Extract(ExtractFunc),

    // Role functions
    CurrentRole,
    CurrentUser,
    SessionUser,
    SystemUser,
    User,

    // JSON functions
    Json(JsonFunc),
    JsonArrayAgg(JsonArrayAgg),
    JsonExists(JsonExistsExpr),
    JsonObject(JsonObjectExpr),
    JsonObjectAgg(JsonObjectAgg),
    JsonQuery(JsonQueryExpr),
    JsonScalar(ExprNode),
    JsonSerialize(JsonSerializeExpr),
    JsonValue(JsonValueFunc),

    // XML functions
    XmlConcat(Vec<ExprNode>),
    XmlElement(XmlElement),
    XmlExists(XmlExists),
    XmlForest(Vec<NamedValue>),
    XmlParse(XmlParse),
    XmlProcessingInstruction(XmlProcessingInstruction),
    XmlRoot(XmlRoot),
    XmlSerialize(XmlSerialize),
}

impl_from!(ExtractFunc for SqlFunction::Extract);
impl_from!(JsonArrayAgg for SqlFunction);
impl_from!(JsonExistsExpr for SqlFunction::JsonExists);
impl_from!(JsonFunc for SqlFunction::Json);
impl_from!(JsonObjectAgg for SqlFunction);
impl_from!(JsonObjectExpr for SqlFunction::JsonObject);
impl_from!(JsonQueryExpr for SqlFunction::JsonQuery);
impl_from!(JsonSerializeExpr for SqlFunction::JsonSerialize);
impl_from!(JsonValueFunc for SqlFunction::JsonValue);
impl_from!(NormalizeFunc for SqlFunction::Normalize);
impl_from!(OverlayFunc for SqlFunction::Overlay);
impl_from!(PositionFunc for SqlFunction::Position);
impl_from!(SubstringFunc for SqlFunction::Substring);
impl_from!(TrimFunc for SqlFunction::Trim);
impl_from!(TypecastExpr for SqlFunction::Typecast);
impl_from!(XmlElement for SqlFunction);
impl_from!(XmlExists for SqlFunction);
impl_from!(XmlParse for SqlFunction);
impl_from!(XmlProcessingInstruction for SqlFunction);
impl_from!(XmlRoot for SqlFunction);
impl_from!(XmlSerialize for SqlFunction);

use pg_basics::impl_from;
use crate::{ExprNode, JsonArrayAgg, JsonObjectAgg};
use crate::ExtractFunc;
use crate::JsonExistsExpr;
use crate::JsonFunc;
use crate::JsonObjectExpr;
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
