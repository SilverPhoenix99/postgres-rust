#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum TableRef {
    XmlTable(XmlTable),
    JsonTable(JsonTable),
    Rows(RowsTableRef),
    Function(FunctionTableRef),
    Relation(RelationTableRef),
    Sample(SampleTableRef),
    Subselect(SubselectTableRef),
}

use crate::FunctionTableRef;
use crate::JsonTable;
use crate::RelationTableRef;
use crate::RowsTableRef;
use crate::SampleTableRef;
use crate::SubselectTableRef;
use crate::XmlTable;
use derive_more::From;
