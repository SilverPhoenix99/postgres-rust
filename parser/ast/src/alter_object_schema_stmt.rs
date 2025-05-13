#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterObjectSchemaStmt {
    target: AlterObjectSchemaTarget,
    new_schema: Str,
}

impl AlterObjectSchemaStmt {
    #[inline(always)]
    pub fn new<T: Into<Str>>(target: AlterObjectSchemaTarget, new_schema: T) -> Self {
        Self {
            target,
            new_schema: new_schema.into(),
        }
    }

    pub fn target(&self) -> &AlterObjectSchemaTarget {
        &self.target
    }

    pub fn new_schema(&self) -> &str {
        &self.new_schema
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterObjectSchemaTarget {
    Aggregate(AggregateWithArgs),
    Collation(QualifiedName),
    Conversion(QualifiedName),
    Domain(QualifiedName),
    Extension(Str),
    ForeignTable { target: RelationExpr, missing_ok: bool },
    Function(FunctionWithArgs),
    MaterializedView { target: QualifiedName, missing_ok: bool },
    Operator(OperatorWithArgs),
    OperatorClass(QualifiedName),
    OperatorFamily(QualifiedName),
    Procedure(FunctionWithArgs),
    Routine(FunctionWithArgs),
    Sequence { target: QualifiedName, missing_ok: bool },
    Statistic(QualifiedName),
    Table { target: RelationExpr, missing_ok: bool },
    TextSearchConfiguration(QualifiedName),
    TextSearchDictionary(QualifiedName),
    TextSearchParser(QualifiedName),
    TextSearchTemplate(QualifiedName),
    Type(QualifiedName),
    View { target: QualifiedName, missing_ok: bool },
}

use crate::AggregateWithArgs;
use crate::FunctionWithArgs;
use crate::OperatorWithArgs;
use crate::QualifiedName;
use crate::RelationExpr;
use postgres_basics::Str;
