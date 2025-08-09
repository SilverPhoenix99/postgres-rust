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

use pg_basics::QualifiedName;
use pg_generic_set_ast::VarValue;
