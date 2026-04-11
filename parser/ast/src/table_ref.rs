#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum TableRef {
    // TODO: GraphTable(GraphTable),
    XmlTable(XmlTable),
    JsonTable(JsonTable),
    Rows(RowsTableRef),
    Function(FunctionTableRef),
    Relation(RelationTableRef),
    Sample(SampleTableRef),
    Subselect(SubselectTableRef),
    Join(Box<JoinExpr>)
}

impl From<JoinExpr> for TableRef {
    fn from(join_expr: JoinExpr) -> Self {
        Self::Join(Box::new(join_expr))
    }
}

use crate::FunctionTableRef;
use crate::JoinExpr;
use crate::JsonTable;
use crate::RelationTableRef;
use crate::RowsTableRef;
use crate::SampleTableRef;
use crate::SubselectTableRef;
use crate::XmlTable;
use derive_more::From;
