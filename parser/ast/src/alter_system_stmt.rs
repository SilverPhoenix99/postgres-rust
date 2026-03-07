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

use crate::VarValue;
use pg_basics::QualifiedName;
