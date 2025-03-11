#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterSystemStmt {
    ResetAll,
    Reset {
        name: QualifiedName
    },
    SetDefault {
        name: QualifiedName
    },
    Set {
        name: QualifiedName,
        values: Vec<VarValue>
    }
}

use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::VarValue;
