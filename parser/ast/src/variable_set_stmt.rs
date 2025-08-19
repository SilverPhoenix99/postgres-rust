#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VariableSetStmt {
    stmt: SetRest,
    is_local: bool,
}

impl VariableSetStmt {
    pub fn local(stmt: SetRest) -> Self {
        Self { stmt, is_local: true }
    }

    pub fn session(stmt: SetRest) -> Self {
        Self { stmt, is_local: false }
    }

    pub fn stmt(&self) -> &SetRest {
        &self.stmt
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }
}

use pg_generic_set_ast::SetRest;
