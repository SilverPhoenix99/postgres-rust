#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum FuncExprWindowless {
    SqlFunction(SqlFunction),
    FuncCall(FuncCall),
}

impl From<JsonArrayAgg> for FuncExprWindowless {
    fn from(value: JsonArrayAgg) -> Self {
        Self::SqlFunction(value.into())
    }
}

impl From<JsonObjectAgg> for FuncExprWindowless {
    fn from(value: JsonObjectAgg) -> Self {
        Self::SqlFunction(value.into())
    }
}

use crate::FuncCall;
use crate::JsonArrayAgg;
use crate::JsonObjectAgg;
use crate::SqlFunction;
use derive_more::From;
