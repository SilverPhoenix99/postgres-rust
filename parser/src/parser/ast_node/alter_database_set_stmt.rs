#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterDatabaseSetStmt {
    name: Str,
    option: AlterdbSetOption
}

impl AlterDatabaseSetStmt {
    pub fn new<T>(name: T, option: AlterdbSetOption) -> Self
    where T: Into<Str>
    {
        Self {
            name: name.into(),
            option
        }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn option(&self) -> &AlterdbSetOption {
        &self.option
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterdbSetOption {
    Set(SetRest),
    Reset(VariableTarget)
}

use crate::parser::ast_node::SetRest;
use crate::parser::ast_node::VariableTarget;
use postgres_basics::Str;
