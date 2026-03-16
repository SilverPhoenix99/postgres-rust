#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum TableRef {
    XmlTable(XmlTable),
    JsonTable(JsonTable),
    Rows(RowsTableRef),
    Function(FunctionTableRef),
    TableSample(SampleTableRef),
    Subselect(SubselectTableRef),
}

use crate::FunctionTableRef;
use crate::JsonTable;
use crate::RowsTableRef;
use crate::SampleTableRef;
use crate::SubselectTableRef;
use crate::XmlTable;
use derive_more::From;
