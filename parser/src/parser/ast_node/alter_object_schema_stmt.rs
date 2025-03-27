#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterObjectSchemaStmt {
    target: AlterObjectSchemaTarget,
    new_schema: Str,
}

impl AlterObjectSchemaStmt {
    #[inline(always)]
    pub fn new(target: AlterObjectSchemaTarget, new_schema: Str) -> Self {
        Self { target, new_schema }
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
    Aggregate(AggregateWithArgtypes),
    Collation(QualifiedName),
    Conversion(QualifiedName),
    Domain(QualifiedName),
    Extension(Str),
    ForeignTable { target: RelationExpr, missing_ok: bool },
    Function(FunctionWithArgtypes),
    MaterializedView { target: QualifiedName, missing_ok: bool },
    Operator(OperatorWithArgtypes),
    OperatorClass(QualifiedName),
    OperatorFamily(QualifiedName),
    Procedure(FunctionWithArgtypes),
    Routine(FunctionWithArgtypes),
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

use crate::parser::ast_node::{
    AggregateWithArgtypes,
    FunctionWithArgtypes,
    OperatorWithArgtypes,
    QualifiedName,
    RelationExpr
};
use postgres_basics::Str;
