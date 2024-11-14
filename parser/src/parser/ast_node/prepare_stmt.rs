#[derive(Debug, Clone, PartialEq)]
pub struct PrepareStmt {
    name: Str,
    arg_types: Vec<SystemType>,
    query: RawStmt,
}

use crate::parser::ast_node::raw_stmt::RawStmt;
use crate::parser::ast_node::SystemType;
use postgres_basics::Str;
