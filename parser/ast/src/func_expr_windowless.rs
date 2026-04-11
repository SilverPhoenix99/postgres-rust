#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum FuncExprWindowless {
    #[from(SqlFunction, JsonArrayAgg, JsonObjectAgg)]
    SqlFunction(SqlFunction),
    #[from]
    FuncCall(FuncCall),
}

use crate::FuncCall;
use crate::JsonArrayAgg;
use crate::JsonObjectAgg;
use crate::SqlFunction;
use derive_more::From;
