#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum TableRef {
    // TODO: GraphTable(GraphTable),
    #[from]
    Xml(XmlTable),
    #[from]
    Json(JsonTable),
    #[from]
    Rows(RowsTableRef),
    #[from]
    Function(FunctionTableRef),
    #[from]
    Relation(RelationTableRef),
    #[from]
    Sample(SampleTableRef),
    #[from]
    Subselect(SubselectTableRef),
    #[from(JoinExpr)]
    Join(Box<JoinExpr>),
    #[from(ParenTableRef)]
    Parenthesized(Box<ParenTableRef>),
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
