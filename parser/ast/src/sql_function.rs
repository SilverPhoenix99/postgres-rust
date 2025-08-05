#[derive(Debug, Clone, Eq, PartialEq, From)]
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
    #[from] Typecast(TypecastExpr),

    // String functions
    #[from] Normalize(NormalizeFunc),
    #[from] Overlay(OverlayFunc),
    #[from] Position(PositionFunc),
    #[from] Substring(SubstringFunc),
    #[from] Trim(TrimFunc),

    // Time functions
    CurrentDate,
    CurrentTime { precision: Option<i32> },
    CurrentTimestamp { precision: Option<i32> },
    LocalTime { precision: Option<i32> },
    LocalTimestamp { precision: Option<i32> },
    #[from] Extract(ExtractFunc),

    // Role functions
    CurrentRole,
    CurrentUser,
    SessionUser,
    SystemUser,
    User,

    // JSON functions
    #[from] Json(JsonFunc),
    #[from] JsonArrayAgg(JsonArrayAgg),
    #[from] JsonExists(JsonExistsExpr),
    #[from] JsonObject(JsonObjectExpr),
    #[from] JsonObjectAgg(JsonObjectAgg),
    #[from] JsonQuery(JsonQueryExpr),
    #[from] JsonScalar(ExprNode),
    #[from] JsonSerialize(JsonSerializeExpr),
    #[from] JsonValue(JsonValueFunc),

    // XML functions
    XmlConcat(Vec<ExprNode>),
    #[from] XmlElement(XmlElement),
    #[from] XmlExists(XmlExists),
    XmlForest(Vec<NamedValue>),
    #[from] XmlParse(XmlParse),
    #[from] XmlProcessingInstruction(XmlProcessingInstruction),
    #[from] XmlRoot(XmlRoot),
    #[from] XmlSerialize(XmlSerialize),
}

use crate::ExprNode;
use crate::ExtractFunc;
use crate::JsonArrayAgg;
use crate::JsonExistsExpr;
use crate::JsonFunc;
use crate::JsonObjectAgg;
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
use derive_more::From;
