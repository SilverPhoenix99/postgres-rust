#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrepareStmt {
    name: Str,
    arg_types: Vec<Type>,
    query: RawStmt,
}

use crate::parser::ast_node::raw_stmt::RawStmt;
use crate::parser::ast_node::Type;
use postgres_basics::Str;
