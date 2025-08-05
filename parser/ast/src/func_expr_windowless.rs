#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FuncExprWindowless {
    SqlFunction(SqlFunction),
    FuncCall(FuncCall),
}

impl_from!(SqlFunction for FuncExprWindowless);
impl_from!(FuncCall for FuncExprWindowless);

use pg_basics::impl_from;
use crate::FuncCall;
use crate::SqlFunction;
