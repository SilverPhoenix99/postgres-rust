#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum FuncExprWindowless {
    SqlFunction(SqlFunction),
    FuncCall(FuncCall),
}

use crate::FuncCall;
use crate::SqlFunction;
use derive_more::From;
