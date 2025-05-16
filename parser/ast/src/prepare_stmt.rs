#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrepareStmt {
    name: Str,
    arg_types: Vec<Type>,
    query: RawStmt,
}

use crate::RawStmt;
use crate::Type;
use pg_basics::Str;
