#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum TableRef {
    // TODO: GraphTable(GraphTable),
    Xml(XmlTable),
    Json(JsonTable),
    Rows(RowsTableRef),
    Function(FunctionTableRef),
    Relation(RelationTableRef),
    Sample(SampleTableRef),
    Subselect(SubselectTableRef),
    Join(Box<JoinExpr>),
    Parenthesized(Box<ParenTableRef>),
}

impl From<JoinExpr> for TableRef {
    fn from(join_expr: JoinExpr) -> Self {
        Self::Join(Box::new(join_expr))
    }
}

impl From<ParenTableRef> for TableRef {
    fn from(value: ParenTableRef) -> Self {
        Self::Parenthesized(Box::new(value))
    }
}

use crate::FunctionTableRef;
use crate::JoinExpr;
use crate::JsonTable;
use crate::ParenTableRef;
use crate::RelationTableRef;
use crate::RowsTableRef;
use crate::SampleTableRef;
use crate::SubselectTableRef;
use crate::XmlTable;
use derive_more::From;
